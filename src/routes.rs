use crate::establish_connection;
use crate::models::{Post, PostError, PostResponse};
use crate::schema::posts;
use axum::Json;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

/// GET /posts
pub async fn list_posts() -> Result<PostResponse, PostError> {
    // Establish a temporary database connection
    let mut conn = establish_connection();

    // Query all posts from the database using Diesel
    posts::table
        .load::<Post>(&mut conn)
        .map(PostResponse::JsonData)
        .map_err(|_| PostError::InternalServerError)
}

/// POST /posts
pub async fn create_post(Json(payload): Json<NewPost>) -> Result<PostResponse, PostError> {
    let mut conn = establish_connection();

    // Insert the new post and return the generated row
    diesel::insert_into(posts::table)
        .values(&payload)
        .get_result::<Post>(&mut conn)
        .map(|_| PostResponse::Created) // Map success to your 201 Created status enum
        .map_err(|_| PostError::InternalServerError)
}
