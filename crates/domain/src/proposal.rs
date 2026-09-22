use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(len_char_min = 5, len_char_max = 255),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ProposalSource(String);

#[nutype(
    validate(predicate = |v| v.has_host() && v.scheme() == "https"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ProposalSourceUrl(url::Url);

impl TryFrom<url::Url> for ProposalSourceUrl {
    type Error = ProposalSourceUrlError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        ProposalSourceUrl::try_new(value)
    }
}
