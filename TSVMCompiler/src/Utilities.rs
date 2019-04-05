pub fn copy_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    vec.clone()
}

pub fn copy<T: Clone>(obj: &T) -> T { obj.clone() }

pub fn is_whitespace(ln: String) -> bool {
    let mut no_ws: String = String::new();
    for ch in ln.chars() {
        match ch {
            '\n' => {},
            '\t' => {},
            '\r' => {},
            ' ' => {},
            _ => no_ws.push(ch)
        }
    }
    no_ws.is_empty()
}

pub fn rem_quotes(s: String) -> String {
    let mut out: Vec<u8> = Vec::new();
    for byte in s.as_bytes() { if *byte != b'"' { out.push(*byte) }}
    String::from_utf8(out).unwrap()
}

pub fn copy_str(s: &String) -> String {
    s.clone()
}