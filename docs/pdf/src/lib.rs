use wasm_bindgen::prelude::*;
use lopdf::{Document, Object, ObjectId, dictionary};
use std::collections::HashMap;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Sanitize PDF bytes by finding the %PDF- header and stripping any leading junk
fn sanitize_pdf_bytes(bytes: &[u8]) -> &[u8] {
    if let Some(pos) = bytes.windows(5).position(|w| w == b"%PDF-") {
        &bytes[pos..]
    } else {
        bytes
    }
}

/// Get the number of pages in a PDF document.
#[wasm_bindgen]
pub fn get_page_count(pdf_bytes: &[u8]) -> Result<u32, JsValue> {
    let clean = sanitize_pdf_bytes(pdf_bytes);
    let doc = Document::load_mem(clean)
        .map_err(|e| JsValue::from_str(&format!("Failed to load PDF: {}", e)))?;
    Ok(doc.get_pages().len() as u32)
}

/// Merge multiple PDFs into one and compress based on quality.
/// quality: 0–100 (100 = minimal compression, lower = more aggressive)
#[wasm_bindgen]
pub fn merge_and_compress(pdf_arrays: &JsValue, quality: u8) -> Result<js_sys::Uint8Array, JsValue> {
    let arrays = js_sys::Array::from(pdf_arrays);
    let count = arrays.length();

    if count == 0 {
        return Err(JsValue::from_str("No PDF files provided"));
    }

    let mut documents: Vec<Document> = Vec::with_capacity(count as usize);
    for i in 0..count {
        let val = arrays.get(i);
        let u8arr = js_sys::Uint8Array::new(&val);
        let bytes = u8arr.to_vec();
        let clean = sanitize_pdf_bytes(&bytes);
        let doc = Document::load_mem(clean)
            .map_err(|e| JsValue::from_str(&format!("Failed to load PDF {}: {}", i + 1, e)))?;
        documents.push(doc);
    }

    let mut doc = if documents.len() == 1 {
        documents.remove(0)
    } else {
        merge_documents(documents)?
    };

    // Apply compression based on quality level
    apply_compression(&mut doc, quality);

    let bytes = save_document(&mut doc)?;
    let result = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    result.copy_from(&bytes);
    Ok(result)
}

/// Apply multi-level compression to a PDF document.
/// quality 0-30:  Aggressive — strip metadata, deduplicate, remove thumbnails/outlines, annotations
/// quality 31-60: Medium — strip metadata, deduplicate, thumbnails
/// quality 61-90: Light — deduplicate, basic compress
/// quality 91-100: Minimal — just basic compress
fn apply_compression(doc: &mut Document, quality: u8) {
    // 1. Always: basic lopdf compress (deflate on uncompressed streams)
    doc.compress();

    // 2. Deduplicate identical stream objects (any quality < 100)
    if quality < 100 {
        deduplicate_streams(doc);
    }

    // 3. Remove unused/unreferenced objects
    if quality < 90 {
        prune_unreferenced_objects(doc);
    }

    // 4. Strip metadata
    if quality < 70 {
        strip_metadata(doc);
    }

    // 5. Strip non-essential objects (thumbnails, outlines/bookmarks, viewer preferences)
    if quality < 50 {
        strip_thumbnails(doc);
        strip_outlines(doc);
    }

    // 6. Aggressive: strip all annotations, form fields, JS actions
    if quality < 30 {
        strip_annotations(doc);
        strip_structure_tree(doc);
    }

    // Final: re-run basic compress to catch anything we decompressed
    doc.compress();
}

/// Deduplicate streams with identical content by replacing duplicates with references.
fn deduplicate_streams(doc: &mut Document) {
    // Hash all stream contents -> first ObjectId with that content
    let mut seen: HashMap<Vec<u8>, ObjectId> = HashMap::new();
    let mut replacements: Vec<(ObjectId, ObjectId)> = Vec::new();

    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();

    for id in &all_ids {
        if let Some(Object::Stream(ref stream)) = doc.objects.get(id) {
            let content = &stream.content;
            if content.len() < 32 { continue; } // skip tiny streams
            if let Some(&original_id) = seen.get(content) {
                if original_id != *id {
                    replacements.push((*id, original_id));
                }
            } else {
                seen.insert(content.clone(), *id);
            }
        }
    }

    // Replace references: for each duplicate, replace all references to it
    for (dup_id, original_id) in &replacements {
        // Remove the duplicate object
        doc.objects.remove(dup_id);

        // Update all references pointing to the duplicate to point to the original
        let all_obj_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
        for obj_id in all_obj_ids {
            if let Some(obj) = doc.objects.get_mut(&obj_id) {
                replace_references_in_object(obj, *dup_id, *original_id);
            }
        }
        // Also update trailer
        let trailer_clone = doc.trailer.clone();
        let mut trailer_obj = Object::Dictionary(trailer_clone);
        replace_references_in_object(&mut trailer_obj, *dup_id, *original_id);
        if let Object::Dictionary(d) = trailer_obj {
            doc.trailer = d;
        }
    }
}

fn replace_references_in_object(obj: &mut Object, from: ObjectId, to: ObjectId) {
    match obj {
        Object::Reference(ref mut r) => {
            if *r == from { *r = to; }
        }
        Object::Array(ref mut arr) => {
            for item in arr.iter_mut() {
                replace_references_in_object(item, from, to);
            }
        }
        Object::Dictionary(ref mut dict) => {
            for (_, val) in dict.iter_mut() {
                replace_references_in_object(val, from, to);
            }
        }
        Object::Stream(ref mut stream) => {
            for (_, val) in stream.dict.iter_mut() {
                replace_references_in_object(val, from, to);
            }
        }
        _ => {}
    }
}

/// Remove objects that aren't referenced by any other object.
fn prune_unreferenced_objects(doc: &mut Document) {
    // Collect all referenced ObjectIds
    let mut referenced: std::collections::HashSet<ObjectId> = std::collections::HashSet::new();

    // Start from trailer
    collect_refs_from_dict(&doc.trailer, &mut referenced);

    // Transitively collect all referenced objects
    let mut frontier: Vec<ObjectId> = referenced.iter().cloned().collect();
    let mut visited: std::collections::HashSet<ObjectId> = std::collections::HashSet::new();

    while let Some(id) = frontier.pop() {
        if !visited.insert(id) { continue; }
        if let Some(obj) = doc.objects.get(&id) {
            collect_refs_from_object(obj, &mut |ref_id| {
                referenced.insert(ref_id);
                frontier.push(ref_id);
            });
        }
    }

    // Remove unreferenced objects
    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in all_ids {
        if !referenced.contains(&id) {
            doc.objects.remove(&id);
        }
    }
}

fn collect_refs_from_dict(dict: &lopdf::Dictionary, refs: &mut std::collections::HashSet<ObjectId>) {
    for (_, val) in dict.iter() {
        collect_refs_from_object(val, &mut |id| { refs.insert(id); });
    }
}

fn collect_refs_from_object(obj: &Object, cb: &mut dyn FnMut(ObjectId)) {
    match obj {
        Object::Reference(id) => cb(*id),
        Object::Array(arr) => {
            for item in arr { collect_refs_from_object(item, cb); }
        }
        Object::Dictionary(dict) => {
            for (_, val) in dict.iter() { collect_refs_from_object(val, cb); }
        }
        Object::Stream(stream) => {
            for (_, val) in stream.dict.iter() { collect_refs_from_object(val, cb); }
        }
        _ => {}
    }
}

/// Strip metadata (Info dictionary, XMP metadata streams).
fn strip_metadata(doc: &mut Document) {
    doc.trailer.remove(b"Info");

    let ids_to_check: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in ids_to_check {
        let is_metadata = match doc.objects.get(&id) {
            Some(Object::Dictionary(dict)) => {
                dict.get(b"Type")
                    .map(|v| matches!(v, Object::Name(ref n) if n == b"Metadata"))
                    .unwrap_or(false)
            }
            Some(Object::Stream(stream)) => {
                stream.dict.get(b"Type")
                    .map(|v| matches!(v, Object::Name(ref n) if n == b"Metadata"))
                    .unwrap_or(false)
                || stream.dict.get(b"Subtype")
                    .map(|v| matches!(v, Object::Name(ref n) if n == b"XML"))
                    .unwrap_or(false)
            }
            _ => false,
        };
        if is_metadata {
            doc.objects.remove(&id);
        }
    }

    // Also remove Metadata references from page and catalog dictionaries
    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in all_ids {
        if let Some(Object::Dictionary(ref mut dict)) = doc.objects.get_mut(&id) {
            dict.remove(b"Metadata");
            dict.remove(b"PieceInfo");
            dict.remove(b"LastModified");
        }
    }
}

/// Remove thumbnail images from pages.
fn strip_thumbnails(doc: &mut Document) {
    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in all_ids {
        if let Some(Object::Dictionary(ref mut dict)) = doc.objects.get_mut(&id) {
            dict.remove(b"Thumb");
        }
    }
}

/// Remove document outline/bookmarks.
fn strip_outlines(doc: &mut Document) {
    // Find and remove Outlines from the catalog
    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in all_ids {
        if let Some(Object::Dictionary(ref mut dict)) = doc.objects.get_mut(&id) {
            // Check if this is the catalog
            let is_catalog = dict.get(b"Type")
                .map(|v| matches!(v, Object::Name(ref n) if n == b"Catalog"))
                .unwrap_or(false);
            if is_catalog {
                dict.remove(b"Outlines");
                dict.remove(b"OpenAction");
                dict.remove(b"PageLabels");
                dict.remove(b"Names");
            }
        }
    }
}

/// Strip annotations from all pages.
fn strip_annotations(doc: &mut Document) {
    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in all_ids {
        if let Some(Object::Dictionary(ref mut dict)) = doc.objects.get_mut(&id) {
            let is_page = dict.get(b"Type")
                .map(|v| matches!(v, Object::Name(ref n) if n == b"Page"))
                .unwrap_or(false);
            if is_page {
                dict.remove(b"Annots");
                dict.remove(b"AA"); // Additional actions
            }
        }
    }
}

/// Strip the document structure tree (accessibility tags — large but not needed for viewing).
fn strip_structure_tree(doc: &mut Document) {
    let all_ids: Vec<ObjectId> = doc.objects.keys().cloned().collect();
    for id in all_ids {
        if let Some(Object::Dictionary(ref mut dict)) = doc.objects.get_mut(&id) {
            let is_catalog = dict.get(b"Type")
                .map(|v| matches!(v, Object::Name(ref n) if n == b"Catalog"))
                .unwrap_or(false);
            if is_catalog {
                dict.remove(b"StructTreeRoot");
                dict.remove(b"MarkInfo");
                dict.remove(b"Lang");
            }
        }
    }
}

/// Merge multiple lopdf Documents into a single Document.
fn merge_documents(documents: Vec<Document>) -> Result<Document, JsValue> {
    let mut merged_doc = Document::with_version("1.7");
    let mut max_id: u32 = 1;
    let mut all_page_refs: Vec<ObjectId> = Vec::new();

    for mut doc in documents {
        doc.renumber_objects_with(max_id as u32);
        max_id = doc.max_id + 1;

        let pages = doc.get_pages();
        let mut page_ids: Vec<(u32, ObjectId)> = pages.into_iter().collect();
        page_ids.sort_by_key(|(num, _)| *num);

        for (_, page_id) in &page_ids {
            all_page_refs.push(*page_id);
        }

        for (id, object) in doc.objects {
            merged_doc.objects.insert(id, object);
        }
    }

    let pages_id = merged_doc.new_object_id();
    let page_refs: Vec<Object> = all_page_refs
        .iter()
        .map(|id| Object::Reference(*id))
        .collect();

    let pages_dict = lopdf::dictionary! {
        "Type" => "Pages",
        "Count" => all_page_refs.len() as u32,
        "Kids" => page_refs,
    };
    merged_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    for page_id in &all_page_refs {
        if let Ok(page_obj) = merged_doc.objects.get_mut(page_id)
            .ok_or_else(|| JsValue::from_str("Page object not found"))
        {
            if let Object::Dictionary(ref mut dict) = page_obj {
                dict.set("Parent", Object::Reference(pages_id));
            }
        }
    }

    let catalog_id = merged_doc.new_object_id();
    let catalog_dict = lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    merged_doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

    merged_doc.trailer.set("Root", Object::Reference(catalog_id));

    merged_doc.max_id = merged_doc.objects.keys()
        .map(|(id, _)| *id)
        .max()
        .unwrap_or(0);

    Ok(merged_doc)
}

/// Save a lopdf Document to bytes.
fn save_document(doc: &mut Document) -> Result<Vec<u8>, JsValue> {
    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .map_err(|e| JsValue::from_str(&format!("Failed to save PDF: {}", e)))?;
    Ok(buf)
}
