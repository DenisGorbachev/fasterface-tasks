use subtype::newtype;
use url::Url;

newtype!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    pub struct FasterfaceUrl(Url);
);

impl FasterfaceUrl {}
