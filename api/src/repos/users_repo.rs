use std::collections;

use crate::services::mongo::get_client;
use anyhow::{anyhow, Result};
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub image_url: Option<String>,
    pub token_expiry: i64,
    pub session_token: Vec<String>,
}

impl UserModel {
    pub fn set_session_token(&mut self, session_token: String) {
        self.session_token.push(session_token);
    }
    pub fn remove_session_token(&mut self, session_token: &str) {
        self.session_token.retain(|t| t != session_token);
    }
}

pub async fn get_users_collection() -> Result<Collection<UserModel>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<UserModel>("users");
    Ok(collection)
}

pub async fn add_user(user: UserModel) -> Result<UserModel> {
    let collection = get_users_collection().await?;
    let res = collection.insert_one(user).await?;
    let id = res
        .inserted_id
        .as_object_id()
        .ok_or(anyhow!("No id found"))?;
    info!("Inserted user with id: {:?}", id);
    let inserted_user = find_user_by_id(&id.to_string()).await;
    inserted_user.ok_or(anyhow!("User not found in db after adding"))
}

pub async fn find_user_by_user_id(user_id: &str) -> Option<UserModel> {
    let collection = get_users_collection().await;
    let collection = collection
        .inspect_err(|e| error!("Error getting collection: {:?}", e))
        .ok()?;
    let filter = doc! { "user_id": user_id };
    let user = collection.find_one(filter).await.ok()?;
    user
}

pub async fn find_user_by_id(_id: &str) -> Option<UserModel> {
    let collection = get_users_collection().await;
    let collection = collection
        .inspect_err(|e| error!("Error getting collection: {:?}", e))
        .ok()?;
    let oid = ObjectId::parse_str(_id)
        .inspect_err(|e| error!("Error parsing id: {:?}", e))
        .ok()?;
    let filter = doc! { "_id":  oid };
    let user = collection.find_one(filter).await.ok().flatten();

    user
}

pub async fn find_user_by_session_token(session_token: &str) -> Option<UserModel> {
    let collection = get_users_collection().await;
    let collection = collection
        .inspect_err(|e| error!("Error getting collection: {:?}", e))
        .ok()?;
    let filter = doc! { "session_token": session_token };
    let user = collection.find_one(filter).await.ok().flatten();
    user
}

pub async fn update_tokens_for_user(
    user_id: &str,
    access_token: String,
    refresh_token: Option<String>,
    token_expiry: i64,
    session_token: String,
) -> Result<UserModel> {
    let collection = get_users_collection().await?;
    let filter = doc! { "user_id": user_id };
    let update = doc! {
        "$set":
            {
                "access_token": access_token.clone(),
                "refresh_token": refresh_token.clone(),
                "token_expiry": token_expiry
            },
        "$push": { "session_token": session_token.clone() }
    };
    collection.update_one(filter, update).await?;
    let user = find_user_by_user_id(user_id)
        .await
        .ok_or(anyhow!("User not found"))?;
    if user.token_expiry != token_expiry {
        anyhow::bail!("Token expiry not updated")
    }
    if user.access_token != access_token {
        anyhow::bail!("Access token not updated")
    }
    if user.refresh_token != refresh_token {
        anyhow::bail!("Refresh token not updated");
    }
    if !user.session_token.contains(&session_token) {
        anyhow::bail!("Refresh token not updated");
    }
    Ok(user)
}

pub async fn logout(user_id: &str, session_token: &str) -> Result<()> {
    let collection = get_users_collection().await?;
    let filter = doc! { "user_id": user_id };
    let update = doc! {
        "$pull": { "session_token": session_token }
    };
    collection.update_one(filter.clone(), update).await?;
    let user = find_user_by_user_id(user_id)
        .await
        .ok_or(anyhow!("User not found"))?;
    if user.session_token.is_empty() {
        let update = doc! {
            "$set": {
            "access_token": "",
            "refresh_token": None::<String>,
            "token_expiry": 0
            }
        };
        collection.update_one(filter, update).await?;
    }
    Ok(())
}
