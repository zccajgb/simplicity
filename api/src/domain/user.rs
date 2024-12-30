use crate::repos::project_repo::create_inbox_for_user;
use crate::repos::project_repo::does_inbox_exist_for_user;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
}

impl User {
    pub async fn does_user_exist(user: &User) -> Result<bool> {
        does_inbox_exist_for_user(user).await
    }

    pub async fn create_user(user: User) -> Result<()> {
        let exists = User::does_user_exist(&user).await?;
        if !exists {
            create_inbox_for_user(user).await?;
        }
        Ok(())
    }
}
