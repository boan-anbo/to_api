use strum::{EnumString, Display, EnumIter};



#[derive(EnumString, Debug, Display, EnumIter)]
pub enum ApiPaths {
    // set the strum string for the enum
    #[strum(serialize = "/add_tos")]
    AddTos,
    #[strum(serialize = "/find_tos")]
    FindTos,
}


// test
#[cfg(test)]
mod test {
    use super::*;
    use strum::IntoEnumIterator;
    // test if api paths is automatically converted to string
    #[test]
    fn test_api_paths_to_string() {
        let api_paths = ApiPaths::AddTos;
        let api_paths_string = api_paths.to_string();
        assert_eq!(api_paths_string, "/add_tos");
    }

    #[test]
    fn all_api_paths_should_have_slash_in_the_begining() {
        // iterate over all api paths
        for api_paths in ApiPaths::iter() {
            // get the string representation of the api path
            let api_paths_string = api_paths.to_string();
            // check if the string starts with a slash
            assert!(api_paths_string.starts_with('/'));
        }
    }
}