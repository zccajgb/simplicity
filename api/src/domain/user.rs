use repos::project_repo::does_inbox_exist_for_user;
use repos::project_repo::create_inbox_for_user;
#[derive(Clone)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn does_user_exist(user: &User) -> Result<()> {
        let does_exist = does_inbox_exist_for_user(user.id)?;
        Ok(())
    }

    pub fn create_user(&user: User) -> Result<ObjectId> {
        if let Ok(_) = User::does_user_exist(&user) {
            anyhow::bail!("User already exists");
        }

        create_inbox_for_user(user.id).await
    }
}
