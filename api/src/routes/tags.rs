use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_all_projects,
        get_all_projects_without_inbox,
        get_project_by_id,
        add_project,
    ]
}

#[get("/tags")]
pub async fn get_all_tags(user: User) -> Json<Vec<Tag>> {
    let tags: Result<_, _> = get_all_tags_for_user(user).await;
    return match tags {
        Ok(tags) => Json(tags),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/tags/<id>")]
pub async fn get_tag_by_id(user: User, id: String) -> Json<Tag> {
    let tag: Result<_, _> = get_tag_by_id_for_user(user, id).await;
    return match tag {
        Ok(tag) => Json(tag),
        Err(e) => Json(Tag::default()),
    };
}

#[post("/tags", data = "<tag>")]
pub async fn add_tag(user: User, tag: Json<Tag>) -> Json<Tag> {
    let tag: Result<_, _> = add_tag_for_user(user, tag.into_inner()).await;
    return match tag {
        Ok(tag) => Json(tag),
        Err(e) => Json(Tag::default()),
    };
}
