pub fn send_simple_string(response: &str) -> String {
    format!("+{}\r\n", response)
}

pub fn send_bulk_string(response: String) -> String {
    format!("${}\r\n{}\r\n", response.len(), response)
}

pub fn send_null_bulk_string() -> String {
    format!("$-1\r\n")
}