fn printer_error(s: &str) -> String {
    let errors = s.chars().filter(|&c| c < 'a'|| c > 'm').count();
    let total = s.len();
    format!("{}/{}", errors, total)
}
