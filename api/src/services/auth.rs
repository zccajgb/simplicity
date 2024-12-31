use crate::domain::user::User;
use anyhow::Result;
use google_jwt_signin::Client;
use log::error;

pub fn validate_token(token: &str) -> Result<()> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = Client::new(&client_id);
    let token_info = client.verify_id_token(token);
    let Ok(_) = token_info else {
        return Err(anyhow::anyhow!("Token not valid"));
    };

    Ok(())
}

pub fn validate_token_and_get_user(token: &str) -> Result<User> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = Client::new(&client_id);
    let token_info = client.verify_id_token(token)
    .map_err(|e| {
        error!("Error verifying token: {:?}", e);
        anyhow::anyhow!("Token not valid")
    })?;
    let user_id = token_info.claims.subject; 
    // let Ok(token_info) = token_info else {
    //     error!("Token not valid: {:?}", token);
    //     return Err(anyhow::anyhow!("Token not valid"));
    // };
    let user = User { id: user_id };
    Ok(user)
}

pub struct ApiKey(pub String);

impl ApiKey {
    pub fn validate(&self) -> Result<()> {
        let api_key = std::env::var("API_KEY")?;
        if self.0 == api_key {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid API key"))
        }
    }
}
