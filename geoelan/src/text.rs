use unicode_segmentation::UnicodeSegmentation;

/// Truncates &str at grapheme cluster boundary at index `max_len`
pub fn truncate_str(s: &str, max_len: usize) -> String {
    let mut g = s.graphemes(true).collect::<Vec<&str>>();
    g.truncate(max_len);
    g.join("")
}
