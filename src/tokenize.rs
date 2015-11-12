pub fn tokenize(text: &str) -> Vec<&str> {
  text.split(matches).filter(|s| s.len() > 0).collect()
}

fn matches(c: char) -> bool {
    match c {
        ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
        | ':' | '\t' | '\n' | '(' | ')' | '-' => true,
        _ => false
    }
}
