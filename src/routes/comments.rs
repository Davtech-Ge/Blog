use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{errors::AppError, models::{self}, Pool};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/{id}/comments").route(web::get().to(user_comment)))
    .service(
        web::resource("/posts/{id}/comments")
            .route(web::post().to(add_comments))
            .route(web::get().to(post_comments)),
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentInput{
    user_id: i32,
    body: String,
}

async fn add_comments(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
    comment: web::Json<CommentInput>
) -> impl Responder {
    let pool = pool.clone();
    let data = comment.into_inner();
    let user_id = data.user_id;
    let body = data.body;

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        models::create_comment(&mut conn, user_id, post_id.into_inner(), body)
    })
    .await
    .map_err(|e|{
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(comment ) => HttpResponse::Ok().json(comment),
        Err(err) => err.error_response(),
    }
}

//getting all comments on a post 
async fn post_comments(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> impl Responder {
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        models::post_comments(&mut conn, post_id.into_inner())
    })
    .await
    .map_err(|e|{
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(err) => err.error_response()
    }
}

async fn user_comment(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let pool = pool.clone();
    let user_id = user_id.into_inner();

    let result = web::block(move|| {
        let mut conn = pool.get().unwrap();
        models::user_comments(&mut conn, user_id)
    })
    .await
    .map_err(|e| {
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(err) => err.error_response(),
    }
}