use subtype::newtype;

newtype!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub struct ObjectId([u8; 20]);
);

impl ObjectId {}
