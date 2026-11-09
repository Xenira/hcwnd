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
    fn query_option(&self) -> Option<String>;
}

#[derive(Debug, Clone)]
pub enum HxMethod {
    Get(String),
    Post(String),
    Put(String),
    Delete(String),
    Query(String),
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

    fn query_option(&self) -> Option<String> {
        match self {
            HxMethod::Query(url) => Some(url.clone()),
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

    fn query_option(&self) -> Option<String> {
        self.as_ref().and_then(|method| method.query_option())
    }
}

pub enum HxSwap {
    InnerHtml,
    OuterHtml,
    TextContent,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    InnerMorph,
    OuterMorph,
    OuterSync,
    Delete,
    None,
    Upsert,
}

impl Display for HxSwap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HxSwap::InnerHtml => write!(f, "innerHTML"),
            HxSwap::OuterHtml => write!(f, "outerHTML"),
            HxSwap::TextContent => write!(f, "textContent"),
            HxSwap::BeforeBegin => write!(f, "beforebegin"),
            HxSwap::AfterBegin => write!(f, "afterbegin"),
            HxSwap::BeforeEnd => write!(f, "beforeend"),
            HxSwap::AfterEnd => write!(f, "afterend"),
            HxSwap::InnerMorph => write!(f, "innerMorph"),
            HxSwap::OuterMorph => write!(f, "outerMorph"),
            HxSwap::OuterSync => write!(f, "outerSync"),
            HxSwap::Delete => write!(f, "delete"),
            HxSwap::None => write!(f, "none"),
            HxSwap::Upsert => write!(f, "upsert"),
        }
    }
}
