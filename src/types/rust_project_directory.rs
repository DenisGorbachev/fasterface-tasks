use subtype::subtype_path_buf;

subtype_path_buf!(
    pub struct RustProjectDirectory(PathBuf);
);

impl RustProjectDirectory {}
