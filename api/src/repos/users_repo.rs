use serde_json::{json, Value};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::couchdb_repo::create_http_client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _rev: Option<String>,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub image_url: Option<String>,
    pub token_expiry: i64,
    pub session_token: Vec<String>,
    pub inbox_id: Option<String>,
}

impl UserModel {
    pub fn set_session_token(&mut self, session_token: String) {
        self.session_token.push(session_token);
    }
    pub fn remove_session_token(&mut self, session_token: &str) {
        self.session_token.retain(|t| t != session_token);
    }
}

pub async fn add_user(user: UserModel) -> Result<UserModel> {
    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);
    let res = client
        .post(format!("{}/users", couch_url))
        .basic_auth(&username, Some(&password))
        .json(&user)
        .send()
        .await?;

    let json = res.json::<Value>().await?;

    let id: &str = json["id"]
        .as_str()
        .ok_or(anyhow::anyhow!("Failed to get id"))?;
    find_user_by_id(id)
        .await?
        .ok_or(anyhow::anyhow!("add_user: User not found"))
}

pub async fn set_inbox_id_for_user(user_id: &str, inbox_id: &str) -> Result<UserModel> {
    let mut user = find_user_by_user_id(user_id).await?.ok_or(anyhow::anyhow!(
        "set_inbox_id_for_user 1: User not found: {:?}",
        user_id
    ))?;

    user.inbox_id = Some(inbox_id.to_string());

    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");
    error!(
        "formatted url: {}/users/{}",
        couch_url,
        user._id.clone().unwrap()
    );
    let client = create_http_client(&couch_url);
    let res = client
        .put(format!("{}/users/{}", couch_url, user._id.clone().unwrap()))
        .basic_auth(&username, Some(&password))
        .json(&user)
        .send()
        .await;

    let json = res?.json::<Value>().await?;

    let user = find_user_by_user_id(user_id)
        .await?
        .ok_or(anyhow::anyhow!("set_inbox_id_for_user 2: User not found"))?;
    Ok(user)
}

pub async fn find_user_by_user_id(user_id: &str) -> Result<Option<UserModel>> {
    let filter = json!({ "selector": { "user_id": user_id } });
    find_user(filter).await
}

async fn find_user(query: Value) -> Result<Option<UserModel>> {
    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);

    let res = client
        .post(format!("{}/users/_find", couch_url))
        .basic_auth(&username, Some(&password))
        .json(&query)
        .send()
        .await?;
    let json = res.json::<Value>().await.ok();
    let Some(json) = json else {
        return Ok(None);
    };
    let user: Vec<UserModel> = serde_json::from_value(json["docs"].clone())
        .inspect_err(|e| error!("Error deserializing user: {:?}", e))?;
    if user.is_empty() {
        return Ok(None);
    }
    Ok(Some(user[0].clone()))
}

pub async fn find_user_by_id(_id: &str) -> Result<Option<UserModel>> {
    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);

    let res = client
        .get(format!("{}/users/{}", couch_url, _id))
        .basic_auth(&username, Some(&password))
        .send()
        .await?;
    let json = res.json::<Value>().await.ok();
    let Some(json) = json else {
        return Ok(None);
    };

    let user: UserModel = serde_json::from_value(json)
        .inspect_err(|e| error!("Error deserializing user in find user by id: {:?}", e))?;
    Ok(Some(user))
}

pub async fn find_user_by_session_token(session_token: &str) -> Option<UserModel> {
    let filter = json!({ "session_token": session_token });
    let user = find_user(filter).await;
    match user {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(e) => {
            error!("Error finding user by session token: {:?}", e);
            None
        }
    }
}

pub async fn update_tokens_for_user(
    user_id: &str,
    access_token: String,
    refresh_token: Option<String>,
    token_expiry: i64,
    session_token: String,
) -> Result<UserModel> {
    let mut user = find_user_by_user_id(user_id)
        .await?
        .ok_or(anyhow::anyhow!("updating_tokens_for_user: User not found"))?;
    user.access_token = access_token;
    user.refresh_token = refresh_token;
    user.token_expiry = token_expiry;
    user.session_token.push(session_token.clone());

    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);
    let res = client
        .put(format!("{}/users/{}", couch_url, user._id.clone().unwrap()))
        .basic_auth(&username, Some(&password))
        .json(&user)
        .send()
        .await?;

    let json = res.json::<Value>().await?;
    if json.get("error").is_some() {
        let error = json.get("reason").and_then(|x| x.as_str());
        return Err(anyhow::anyhow!(
            "Error updating tokens for user: {:?}",
            error.unwrap_or("unknown error")
        ));
    }
    let user = find_user_by_user_id(user_id)
        .await?
        .ok_or(anyhow::anyhow!("update_tokens_for_user: User not found"))?;
    Ok(user)
}

pub async fn logout(user_id: &str, session_token: &str) -> Result<UserModel> {
    let mut user = find_user_by_user_id(user_id)
        .await?
        .ok_or(anyhow::anyhow!("logout: User not found"))?;

    user.session_token.retain(|x| *x != session_token);

    if user.session_token.is_empty() {
        user.access_token = "".to_string();
        user.refresh_token = None;
        user.token_expiry = 0;
    }

    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);
    let res = client
        .put(format!("{}/users/{}", couch_url, user._id.clone().unwrap()))
        .basic_auth(&username, Some(&password))
        .json(&user)
        .send()
        .await;

    res.inspect_err(|e| error!("Error updating inbox id for user: {:?}", e))?;

    let user = find_user_by_user_id(user_id)
        .await?
        .ok_or(anyhow::anyhow!("logout: User not found"))?;
    Ok(user)
}
