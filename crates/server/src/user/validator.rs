const MIN_PASSWORD_LENGTH: usize = 12;
const MIN_NAME_LENGTH: usize = 3;
const MAX_NAME_LENGTH: usize = 32;

pub fn is_valid_email(email: &str) -> bool {
    mailchecker::is_valid(email)
}

pub fn is_valid_password(password: &str) -> bool {
    password.len() >= MIN_PASSWORD_LENGTH
}

pub fn is_valid_name(name: &str) -> bool {
    if name
        .chars()
        .any(|c| !c.is_alphanumeric() || c.is_whitespace())
    {
        return false;
    }

    let len = name.chars().count();
    (MIN_NAME_LENGTH..=MAX_NAME_LENGTH).contains(&len)
}
