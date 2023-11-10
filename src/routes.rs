use crate::establish_connection;
use crate::models::Post;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let mut connection = establish_connection();


    let posts_result = posts.load::<Post>(&mut connection);

    match posts_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
