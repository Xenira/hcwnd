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

pub trait HxMethodOption {
    fn get_option(&self) -> Option<String>;
    fn post_option(&self) -> Option<String>;
    fn put_option(&self) -> Option<String>;
    fn delete_option(&self) -> Option<String>;
}

pub enum HxMethod {
    Get(String),
    Post(String),
    Put(String),
    Delete(String),
}

impl HxMethodOption for HxMethod {
    fn get_option(&self) -> Option<String> {
        match self {
            HxMethod::Get(url) => Some(url.clone()),
            _ => None,
        }
    }

    fn post_option(&self) -> Option<String> {
        match self {
            HxMethod::Post(url) => Some(url.clone()),
            _ => None,
        }
    }

    fn put_option(&self) -> Option<String> {
        match self {
            HxMethod::Put(url) => Some(url.clone()),
            _ => None,
        }
    }

    fn delete_option(&self) -> Option<String> {
        match self {
            HxMethod::Delete(url) => Some(url.clone()),
            _ => None,
        }
    }
}

impl<T> HxMethodOption for Option<T>
where
    T: HxMethodOption,
{
    fn get_option(&self) -> Option<String> {
        self.as_ref().and_then(|method| method.get_option())
    }

    fn post_option(&self) -> Option<String> {
        self.as_ref().and_then(|method| method.post_option())
    }

    fn put_option(&self) -> Option<String> {
        self.as_ref().and_then(|method| method.put_option())
    }

    fn delete_option(&self) -> Option<String> {
        self.as_ref().and_then(|method| method.delete_option())
    }
}
