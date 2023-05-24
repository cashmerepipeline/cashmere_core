use dependencies_sync::tonic::metadata::MetadataMap;

pub fn get_auth_token(metadata: &MetadataMap) -> Option<String> {
    match metadata.get("authorization") {
        Some(token) => {
            let auth_string = token.to_str().unwrap().to_string();
            // 分开Bearer 和 token
            let split_strings: Vec<&str> = auth_string.split(' ').collect();
            let token_string = split_strings[1].to_string();

            Some(token_string)
        }
        None => None,
    }
}
