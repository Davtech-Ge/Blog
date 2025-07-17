use crate::errors::AppError;
use crate::{models, Pool};
use actix_web::{web, HttpResponse, Responder, ResponseError};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to(create_user)))
        .service(web::resource("/users/find/{name}").route(web::get().to(find_user)))
        .service(web::resource("/users/{user_id}").route(web::get().to(get_user)));
}


#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}


// async fn create_user(
//     item: web::Json<UserInput>,
//     pool: web::Data<Pool>
// ) -> impl Responder {
//     let username = item.into_inner().username;
//     let result =  web::block(move || {
//         let conn = &pool.get().unwrap();
        

//         models::create_user(conn, username.as_str())
//     })
//     .await;

//     match result {
//         Ok(user) =>HttpResponse::Ok().json(user),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
//     }
// }

async fn create_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>
) -> impl Responder {
    let username = item.into_inner().username;
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        models::create_user(&mut conn, username.as_str())
    })
    .await
    .map_err(|e| {
        log::error!("Blocking error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err.error_response(),
    }
}


// async fn find_user(
//     name: web::Path<String>,
//     pool: web::Data<Pool>
// ) -> impl Responder {
//      let name = name.into_inner();
//      let result = web::block( move|| {
//         let conn = &pool.get().unwrap();
//         let key = models::UserKey::Username(name.as_str());

//         models::find_user(conn, key)
//     })
//     .await;

//     match result {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(e) => {
//             let app_error = match e {
//             actix_web::error::BlockingError::Error(inner) => inner,
//             actix_web::error::BlockingError::Canceled => AppError::OperationCanceled,
//         };
//         app_error.error_response()
//     }
        
//     }
// }

async fn find_user(
    name: web::Path<String>,
    pool: web::Data<Pool>
) -> impl Responder {
    let name = name.into_inner();
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let key = models::UserKey::Username(name.as_str());

        models::find_user( &mut conn, key)
    })
    .await
    .map_err(|e| {
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err.error_response()
    }
}


// async fn get_user(
//     user_id: web::Path<i32>,
//     pool: web::Data<Pool>
// ) -> impl Responder {
//     let id = user_id.into_inner();
//     let result = web::block(move || {
//         let conn = &pool.get().unwrap();
//         let key = models::UserKey::ID(id);

//         models::find_user(conn, key)
//     })
//     .await;

//     match result {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(e) => {
//             let app_error = match e {
//                 actix_web::error::BlockingError::Error(inner) => inner,
//                 BlockingError::Canceled => AppError::OperationCanceled,
//             };
//             app_error.error_response()
//         }
//     }
// }

async fn get_user(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> impl Responder {
    let id = user_id.into_inner();
    let pool = pool.clone();

    let result = web::block(move || {
        let mut conn  =  pool.get().unwrap();
        let key = models::UserKey::ID(id);

        models::find_user(&mut conn, key)
    })
    .await
    .map_err(|e| {
        log::error!("Blocking Error: {:?}", e);
        AppError::OperationCanceled
    });

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err.error_response()
    }
}