use subtype::subtype;
use url::Url;

subtype!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    pub struct FasterfaceUrl(Url);
);

impl FasterfaceUrl {}
