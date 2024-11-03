use anyhow::Result;
use google_jwt_verify::Client;

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
    return Ok(());
}

pub fn validate_token_and_get_user(token: &str) -> Result<User> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = Client::new(&client_id);
    let token_info = client.verify_id_token(token);
    let Ok(token_info) = token_info else {
        return Err(anyhow::anyhow!("Token not valid"));
    };
    let user = User {
        id: token_info.get_claims().get_subject(),
    };
    return Ok(user);
}
