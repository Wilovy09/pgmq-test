use regex::Regex;

pub fn is_valid_password(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }

    let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(password);
    let has_special = Regex::new(r"[!@#$%^&*(),.?|<>]")
        .unwrap()
        .is_match(password);

    has_uppercase && has_special
}
