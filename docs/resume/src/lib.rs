use std::collections::HashSet;

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn init_console_panic_hook() {
    console_error_panic_hook::set_once();
}

fn tokenize(value: &str) -> Vec<String> {
    value
        .split(|ch: char| !ch.is_alphanumeric())
        .filter_map(|token| {
            let clean = token.trim().to_ascii_lowercase();
            if clean.len() < 3 {
                return None;
            }

            if STOP_WORDS.contains(&clean.as_str()) {
                return None;
            }

            Some(clean)
        })
        .collect()
}

fn top_keywords(job_description: &str, max_count: usize) -> Vec<String> {
    let mut freq = std::collections::HashMap::<String, usize>::new();

    for token in tokenize(job_description) {
        *freq.entry(token).or_insert(0) += 1;
    }

    let mut entries: Vec<(String, usize)> = freq.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(max_count);

    entries.into_iter().map(|(word, _)| word).collect()
}

#[wasm_bindgen]
pub fn analyze_resume(resume_text: String, job_description: String) -> Result<JsValue, JsValue> {
    let resume_tokens: HashSet<String> = tokenize(&resume_text).into_iter().collect();
    let keywords = top_keywords(&job_description, 30);

    let mut matched: Vec<String> = Vec::new();
    let mut missing: Vec<String> = Vec::new();

    for key in &keywords {
        if resume_tokens.contains(key) {
            matched.push(key.clone());
        } else {
            missing.push(key.clone());
        }
    }

    let keyword_score = if keywords.is_empty() {
        0.0
    } else {
        (matched.len() as f64 / keywords.len() as f64) * 100.0
    };

    let bullets = resume_text.matches('•').count()
        + resume_text.matches("- ").count()
        + resume_text.matches("\n*").count();

    let action_verbs = count_action_verbs(&resume_text);

    let structure_bonus = if bullets >= 4 { 8.0 } else { 0.0 };
    let verb_bonus = if action_verbs >= 8 { 7.0 } else { (action_verbs as f64).min(7.0) };

    let score = (keyword_score * 0.85 + structure_bonus + verb_bonus).min(100.0).round() as i32;

    let suggestions = Array::new();
    if missing.len() > 0 {
        suggestions.push(&JsValue::from_str("Add missing job keywords naturally inside experience bullets."));
    }
    if bullets < 4 {
        suggestions.push(&JsValue::from_str("Use more bullet points to make accomplishments easier for ATS and recruiters to scan."));
    }
    if action_verbs < 6 {
        suggestions.push(&JsValue::from_str("Start more bullet points with action verbs (built, led, optimized, delivered)."));
    }

    let matched_array = Array::new();
    for value in matched {
        matched_array.push(&JsValue::from_str(&value));
    }

    let missing_array = Array::new();
    for value in missing {
        missing_array.push(&JsValue::from_str(&value));
    }

    let result = Object::new();
    Reflect::set(&result, &JsValue::from_str("ats_score"), &JsValue::from_f64(score as f64))?;
    Reflect::set(
        &result,
        &JsValue::from_str("keywords_total"),
        &JsValue::from_f64(keywords.len() as f64),
    )?;
    Reflect::set(
        &result,
        &JsValue::from_str("keywords_matched"),
        &JsValue::from_f64(matched_array.length() as f64),
    )?;
    Reflect::set(&result, &JsValue::from_str("matched_keywords"), &matched_array)?;
    Reflect::set(&result, &JsValue::from_str("missing_keywords"), &missing_array)?;
    Reflect::set(&result, &JsValue::from_str("suggestions"), &suggestions)?;

    Ok(result.into())
}

fn count_action_verbs(text: &str) -> usize {
    let lower = text.to_ascii_lowercase();
    ACTION_VERBS
        .iter()
        .filter(|verb| lower.contains(*verb))
        .count()
}

const STOP_WORDS: &[&str] = &[
    "the", "and", "for", "with", "that", "you", "your", "are", "was", "were", "this", "from",
    "have", "has", "had", "but", "not", "all", "our", "their", "about", "into", "out", "use",
    "using", "used", "can", "will", "its", "they", "them", "his", "her", "she", "him", "who",
    "what", "when", "where", "how", "why", "also", "more", "most", "than", "then", "over", "under",
];

const ACTION_VERBS: &[&str] = &[
    "built", "developed", "created", "delivered", "optimized", "improved", "implemented", "designed",
    "launched", "led", "managed", "automated", "scaled", "reduced", "increased", "analyzed", "migrated",
    "architected", "streamlined", "collaborated", "owned", "drove", "executed",
];
