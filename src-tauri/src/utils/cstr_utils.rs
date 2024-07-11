use url::Url;

use crate::utils;

use utils::error_utils::DbError;


pub struct Cstring{
    pub server: String,
    pub user: String,
    pub password: String
}

impl Cstring {
    
    pub fn new(cstr: &str) -> Result<Self, DbError> {
        let url_string = cstr.replace(";", "&").replace(" ", "%20").replace("Server", "server").replace("User Id", "user").replace("Password", "password");
        let url = Url::parse(&format!("sqlserver://{}", url_string))?;
        
        let server = url.host_str().unwrap_or_default();
        let user = url.query_pairs().find(|(key, _)| key == "user").map(|(_, value)| value).unwrap_or_default();
        let password = url.query_pairs().find(|(key, _)| key == "password").map(|(_, value)| value).unwrap_or_default();

        Ok(Self {
            server: server.to_string(),
            user: user.to_string(),
            password: password.to_string()
        })
    }

}

