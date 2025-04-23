use serde_json::{json, Value};

use crate::routes::users::User;
use anyhow::Result;
use reqwest::{
    self,
    header::{self, CONTENT_TYPE, REFERER, USER_AGENT},
    Client,
};

pub fn create_http_client(uri: &str) -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(USER_AGENT, header::HeaderValue::from_static("reqwest"));
    // headers.insert(
    //     CONTENT_TYPE,
    //     header::HeaderValue::from_static("multipart/form-data"),
    // );
    headers.insert(REFERER, header::HeaderValue::from_str(uri).unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build reqwest client");
    client
}

pub async fn create_db_for_user(user: User) -> Result<()> {
    let security_doc = json!({
        "members": {
            "names": [user.user_id],
            "roles": []
        }
    });

    create_db_and_update_security(&user.user_id, &security_doc, "tasks").await?;
    create_db_and_update_security(&user.user_id, &security_doc, "projects").await?;
    create_db_and_update_security(&user.user_id, &security_doc, "tags").await?;
    info!("Created dbs for user: {}", user.user_id);
    Ok(())
}

async fn create_db_and_update_security(
    user_id: &str,
    security_doc: &Value,
    db_name: &str,
) -> Result<()> {
    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);
    let res = client
        .put(format!("{}/{}_{}", couch_url, db_name, user_id))
        .basic_auth(&username, Some(&password))
        .send()
        .await;
    res.inspect_err(|e| error!("Error creating db: {:?}", e))?;

    let res = client
        .put(format!("{}/{}_{}/_security", couch_url, db_name, user_id))
        .basic_auth(&username, Some(&password))
        .json(&security_doc)
        .send()
        .await;
    res.inspect_err(|e| error!("Error updating security: {:?}", e))?;

    Ok(())
}

pub async fn create_inbox_for_user(user: User) -> Result<String> {
    let project = json! ({
        "name": "inbox",
        "completed": false,
    });

    let username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER must be set");
    let password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD must be set");
    let couch_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL must be set");

    let client = create_http_client(&couch_url);

    let res = client
        .post(format!("{}/projects_{}", couch_url, user.user_id))
        .basic_auth(&username, Some(&password))
        .json(&project)
        .send()
        .await?;
    let json = res.json::<Value>().await?;
    let id: &str = json["id"]
        .as_str()
        .ok_or(anyhow::anyhow!("Creating db for user failed to get id"))?;
    Ok(id.to_string())
}
