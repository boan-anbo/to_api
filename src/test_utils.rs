// test module
#[cfg(test)]
pub mod test_utils {
    use std::path::PathBuf;

    pub fn get_test_folder_path() -> PathBuf {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test");
        d

    }

    pub fn get_test_folder_path_string() -> String {
        let folder_path = get_test_folder_path();
        folder_path.into_os_string().into_string().unwrap()
    }

}