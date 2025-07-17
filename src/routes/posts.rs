use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::prelude::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service( web::resource("/user/{id}/posts")
                                    .route(web::post().to(add_posts))
                                    .route(web::get().to(user_posts)))
        .service(web::resource("/posts").route(web::get().to(all_post)))
        .service(web::resource("/posts/{id}/publish").route(web::post().to(publish_post)));
}

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String
}

async fn add_posts(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>
) -> impl Responder {
    let user_id = user_id.into_inner();
    let post = post.into_inner();
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let key = models::UserKey::ID(user_id);
        models::find_user(&mut conn, key).and_then(|user| {
            models::create_post(&mut conn, &user, post.title, post.body)
        })
    })
    .await
    .map_err(|e| {
        log::error!("Blocking error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => err.error_response(),
    }
}

async fn publish_post(post_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        models::publish_post(&mut conn, post_id.into_inner())
        
    })
    .await
    .map_err(|e| {
        log::error!("Blocking error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => err.error_response(),
    }
}

async fn user_posts(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        models::user_posts(&mut conn, user_id.into_inner())
    })
    .await
    .map_err(|e| {
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => err.error_response()
    }
}

async fn all_post(pool: web::Data<Pool>) -> impl Responder {
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        models::all_posts(&mut conn)
    })
    .await
    .map_err(|e| {
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(post ) => HttpResponse::Ok().json(post),
        Err(err) => err.error_response(),
    }
}