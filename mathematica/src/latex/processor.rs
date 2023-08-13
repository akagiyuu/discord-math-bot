use anyhow::Result;

fn shift_start(s: &str, new_start: char) -> &str {
    match s.char_indices().find(|(_, element)| *element == new_start) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
pub fn trim(latex: &[u8]) -> Result<String> {
    let latex = std::str::from_utf8(latex)?;
    let latex = shift_start(latex, '%').to_string();
    Ok(latex.replacen(
        "documentclass{article}",
        "documentclass[border=2pt,varwidth]{standalone}",
        1,
    ))
}
