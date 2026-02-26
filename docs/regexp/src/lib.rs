use wasm_bindgen::prelude::*;
use grex::RegExpBuilder;
use regex::Regex;

/// Exhaustive combinatorial search for the minimal regex that matches all inputs.
///
/// Strategy:
/// 1. Define a set of independent boolean grex options (conversion flags, anchors, etc.)
/// 2. Enumerate all 2^N combinations of these flags (with pruning of conflicting combos)
/// 3. For each combo, build a regex via grex
/// 4. Validate each result against the original inputs using the `regex` crate
/// 5. Pick the shortest valid regex
/// 6. Apply post-processing simplifications to compress further

#[wasm_bindgen]
pub fn generate_regex(match_strings: JsValue) -> String {
    let matches: Vec<String> = serde_wasm_bindgen::from_value(match_strings).unwrap_or_default();

    let valid_matches: Vec<&str> = matches.iter()
        .map(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .collect();

    if valid_matches.is_empty() {
        return String::new();
    }

    // ── Flag definitions ──────────────────────────────────────────────
    // Each flag corresponds to a grex builder method. We enumerate all
    // meaningful subsets. Some flags conflict with each other (e.g.
    // conv_words vs conv_non_words); we prune those.
    #[derive(Clone, Copy, Default)]
    struct Flags {
        conv_digits: bool,
        conv_non_digits: bool,
        conv_words: bool,
        conv_non_words: bool,
        conv_whitespace: bool,
        conv_non_whitespace: bool,
        conv_repetitions: bool,
        case_insensitive: bool,
        capturing_groups: bool,
        without_anchors: bool,
        without_start_anchor: bool,
        without_end_anchor: bool,
        min_repetitions_2: bool,     // use min_repetitions = 2 (vs default 1)
        min_substr_len_2: bool,      // use min_substring_length = 2 (vs default 1)
    }

    const NUM_FLAGS: usize = 14;

    fn flags_from_bits(bits: u32) -> Flags {
        Flags {
            conv_digits:          bits & (1 << 0) != 0,
            conv_non_digits:      bits & (1 << 1) != 0,
            conv_words:           bits & (1 << 2) != 0,
            conv_non_words:       bits & (1 << 3) != 0,
            conv_whitespace:      bits & (1 << 4) != 0,
            conv_non_whitespace:  bits & (1 << 5) != 0,
            conv_repetitions:     bits & (1 << 6) != 0,
            case_insensitive:     bits & (1 << 7) != 0,
            capturing_groups:     bits & (1 << 8) != 0,
            without_anchors:      bits & (1 << 9) != 0,
            without_start_anchor: bits & (1 << 10) != 0,
            without_end_anchor:   bits & (1 << 11) != 0,
            min_repetitions_2:    bits & (1 << 12) != 0,
            min_substr_len_2:     bits & (1 << 13) != 0,
        }
    }

    /// Quick check: prune obviously conflicting or redundant combinations
    fn is_valid_combo(f: &Flags) -> bool {
        // digits + non_digits simultaneously is contradictory
        if f.conv_digits && f.conv_non_digits { return false; }
        // words + non_words simultaneously is contradictory
        if f.conv_words && f.conv_non_words { return false; }
        // whitespace + non_whitespace simultaneously is contradictory
        if f.conv_whitespace && f.conv_non_whitespace { return false; }
        // without_anchors subsumes the individual anchor flags
        if f.without_anchors && (f.without_start_anchor || f.without_end_anchor) { return false; }
        // min_repetitions / min_substr_len only matter when conv_repetitions is on
        if !f.conv_repetitions && (f.min_repetitions_2 || f.min_substr_len_2) { return false; }
        true
    }

    /// Build regex string from a flag combination using grex
    fn build_regex(f: &Flags, inputs: &[&str]) -> Option<String> {
        std::panic::catch_unwind(|| {
            let mut b = RegExpBuilder::from(inputs);

            if f.conv_digits          { b.with_conversion_of_digits(); }
            if f.conv_non_digits      { b.with_conversion_of_non_digits(); }
            if f.conv_words           { b.with_conversion_of_words(); }
            if f.conv_non_words       { b.with_conversion_of_non_words(); }
            if f.conv_whitespace      { b.with_conversion_of_whitespace(); }
            if f.conv_non_whitespace  { b.with_conversion_of_non_whitespace(); }
            if f.conv_repetitions     { b.with_conversion_of_repetitions(); }
            if f.case_insensitive     { b.with_case_insensitive_matching(); }
            if f.capturing_groups     { b.with_capturing_groups(); }
            if f.without_anchors      { b.without_anchors(); }
            if f.without_start_anchor { b.without_start_anchor(); }
            if f.without_end_anchor   { b.without_end_anchor(); }

            if f.min_repetitions_2 {
                b.with_minimum_repetitions(2);
            }
            if f.min_substr_len_2 {
                b.with_minimum_substring_length(2);
            }

            b.build()
        }).ok()
    }

    /// Validate that a regex matches every input string (full match)
    fn validates(regex_str: &str, inputs: &[&str]) -> bool {
        // For anchored patterns (^...$), use them as-is.
        // For un-anchored patterns, wrap with ^...$ to check full match.
        let pattern = if regex_str.starts_with('^') && regex_str.ends_with('$') {
            regex_str.to_string()
        } else if regex_str.starts_with('^') {
            format!("{}$", regex_str)
        } else if regex_str.ends_with('$') {
            format!("^{}", regex_str)
        } else {
            format!("^(?:{})$", regex_str)
        };

        match Regex::new(&pattern) {
            Ok(re) => inputs.iter().all(|s| re.is_match(s)),
            Err(_) => false,
        }
    }

    // ── Main search ───────────────────────────────────────────────────
    let total_combos: u32 = 1 << NUM_FLAGS; // 16384 combinations
    let mut best: Option<String> = None;

    for bits in 0..total_combos {
        let flags = flags_from_bits(bits);
        if !is_valid_combo(&flags) { continue; }

        if let Some(regex_str) = build_regex(&flags, &valid_matches) {
            if regex_str.is_empty() { continue; }

            // Validate correctness: the regex must match all inputs
            if !validates(&regex_str, &valid_matches) { continue; }

            // Compare length - keep the shortest
            let dominated = match &best {
                None => true,
                Some(prev) => regex_str.len() < prev.len(),
            };
            if dominated {
                best = Some(regex_str);
            }
        }
    }

    // ── Post-processing ───────────────────────────────────────────────
    if let Some(r) = best {
        return post_process(&r);
    }

    // Final fallback: simplest possible grex build
    let mut builder = RegExpBuilder::from(&valid_matches);
    builder
        .with_conversion_of_digits()
        .with_conversion_of_words()
        .with_conversion_of_repetitions();
    post_process(&builder.build())
}

/// Apply post-processing simplifications to a regex string
fn post_process(regex: &str) -> String {
    let mut result = regex.to_string();

    // Remove redundant non-capturing group wrapping the entire expression
    // e.g. ^(?:foo)$ → ^foo$
    if result.starts_with("^(?:") && result.ends_with(")$") {
        let inner = &result[4..result.len() - 2];
        // Only unwrap if the inner doesn't contain un-grouped alternation
        if !has_top_level_pipe(inner) {
            result = format!("^{}$", inner);
        }
    }

    // Simplify character classes with single entry: [a] → a
    result = simplify_single_char_classes(&result);

    // Collapse consecutive identical quantified groups
    result = collapse_consecutive(&result);

    result
}

/// Check if a string has a pipe '|' at the top level (not inside parens/brackets)
fn has_top_level_pipe(s: &str) -> bool {
    let mut depth = 0i32;
    let mut in_bracket = false;
    for ch in s.chars() {
        match ch {
            '[' if !in_bracket => in_bracket = true,
            ']' if in_bracket => in_bracket = false,
            '(' if !in_bracket => depth += 1,
            ')' if !in_bracket => depth -= 1,
            '|' if !in_bracket && depth == 0 => return true,
            _ => {}
        }
    }
    false
}

/// Simplify [x] → x for single literal characters (not special)
fn simplify_single_char_classes(s: &str) -> String {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut result = Vec::with_capacity(len);
    let mut i = 0;

    while i < len {
        if bytes[i] == b'[' && i + 2 < len && bytes[i + 2] == b']' {
            let ch = bytes[i + 1];
            // Only simplify if it's a plain alphanumeric or simple char
            if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b' ' {
                result.push(ch);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }

    String::from_utf8(result).unwrap_or_else(|_| s.to_string())
}

/// Collapse patterns like (\d)(\d) → \d{2}  (simple consecutive identical tokens)
fn collapse_consecutive(s: &str) -> String {
    // This is a lightweight optimization: find runs of consecutive identical
    // simple tokens (\d, \w, \s, \D, \W, \S, .) and convert to quantifiers.
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(s.len());
    let mut i = 0;

    while i < len {
        // Check for backslash-escaped tokens: \d, \w, \s, \D, \W, \S
        if i + 1 < len && chars[i] == '\\' && "dwsDWS".contains(chars[i + 1]) {
            let token_char = chars[i + 1];
            let mut count = 1usize;
            let mut j = i + 2;
            // Count consecutive identical tokens
            while j + 1 < len && chars[j] == '\\' && chars[j + 1] == token_char {
                // Make sure the next token isn't already quantified
                if j + 2 < len && (chars[j + 2] == '{' || chars[j + 2] == '*'
                    || chars[j + 2] == '+' || chars[j + 2] == '?') {
                    break;
                }
                count += 1;
                j += 2;
            }
            if count > 1 {
                result.push('\\');
                result.push(token_char);
                result.push_str(&format!("{{{}}}", count));
            } else {
                result.push('\\');
                result.push(token_char);
            }
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}
