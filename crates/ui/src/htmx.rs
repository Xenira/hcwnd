use std::fmt::Display;

pub enum HxEncoding {
    UrlEncoded,
    MultipartFormData,
}

impl Display for HxEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HxEncoding::UrlEncoded => write!(f, "application/x-www-form-urlencoded"),
            HxEncoding::MultipartFormData => write!(f, "multipart/form-data"),
        }
    }
}
