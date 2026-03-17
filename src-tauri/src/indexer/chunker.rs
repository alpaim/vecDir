use text_splitter::TextSplitter;

pub struct TextChunk {
    pub content: String,
    pub start_char_idx: i32,
    pub end_char_idx: i32,
}

const CHUNK_SIZE: usize = 1024;
const OVERLAP_CHARS: usize = 150;

pub fn chunk_text(text: &str) -> Vec<TextChunk> {
    if text.trim().is_empty() {
        return vec![];
    }

    let splitter = TextSplitter::new(CHUNK_SIZE);
    let raw_chunks: Vec<&str> = splitter.chunks(text).collect();

    if raw_chunks.is_empty() {
        return vec![];
    }

    let mut chunks: Vec<TextChunk> = Vec::new();
    let mut search_start: usize = 0;

    for (i, &chunk_content) in raw_chunks.iter().enumerate() {
        let start_idx = find_char_position(text, search_start, chunk_content);
        let end_idx = start_idx + chunk_content.len();

        let content = if i > 0 && !chunks.is_empty() {
            let overlap_start = chunks[i - 1].content.len().saturating_sub(OVERLAP_CHARS);
            let overlap_text = &chunks[i - 1].content[overlap_start..];
            format!("{}{}", overlap_text, chunk_content)
        } else {
            chunk_content.to_string()
        };

        chunks.push(TextChunk {
            content,
            start_char_idx: start_idx as i32,
            end_char_idx: end_idx as i32,
        });

        search_start = end_idx;
    }

    chunks
}

fn find_char_position(text: &str, start_from: usize, needle: &str) -> usize {
    let start = start_from.min(text.len());
    let search_text = &text[start..];

    search_text
        .find(needle)
        .map(|pos| start + pos)
        .unwrap_or(start)
}

pub fn create_empty_chunk(filename: &str) -> TextChunk {
    TextChunk {
        content: format!("file: {}", filename),
        start_char_idx: 0,
        end_char_idx: 0,
    }
}
