use subtype::subtype_string;
use subtype::{IsEmpty, Not};

// TODO: Add crate name validation
subtype_string! {
    pub struct KrateName(String | Not<IsEmpty>)
}
