use unicode_segmentation::UnicodeSegmentation;

pub fn truncate(text: &str, max_bytes: usize) -> (String, String) {
    let mut char_count = 0;
    let mut result = String::new();
    let mut remaining = String::new();

    for grapheme in text.graphemes(true) {
        char_count += grapheme.chars().count();
        if char_count > max_bytes {
            remaining.push_str(grapheme);
        } else {
            result.push_str(grapheme);
        }
    }

    (result, remaining)
}

pub fn into_blocks(text: &str, max_bytes: usize) -> Vec<String> {
    let mut blocks = Vec::new();
    let (mut text, mut remaining) = truncate(text, max_bytes);

    loop {
        blocks.push(text);

        if remaining.is_empty() {
            break;
        }

        (text, remaining) = truncate(&remaining, max_bytes);
    }

    blocks
}
