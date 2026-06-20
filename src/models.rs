use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

enum PostResponse {
    OK,
    Created,
    JsonData(Vec<Post>),
}

impl IntoResponse for PostResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

enum PostError {
    BadRequest,
    InternalServerError,
}
