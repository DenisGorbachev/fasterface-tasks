use subtype::newtype_path_buf;

newtype_path_buf!(
    pub struct RustProjectDirectory(PathBuf);
);

impl RustProjectDirectory {}
