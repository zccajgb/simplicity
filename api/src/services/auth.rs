use anyhow::Result;
use google_jwt_signin::Client;

#[derive(Clone)]
pub struct User {
    pub id: String,
}

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
    let token_info = client.verify_id_token(token);
    let Ok(token_info) = token_info else {
        return Err(anyhow::anyhow!("Token not valid"));
    };
    let user = User {
        id: token_info.claims.subject,
    };
    Ok(user)
}
