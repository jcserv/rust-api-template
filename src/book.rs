use crate::{
    model::BookModel,
    schema::{CreateBookSchema, UpdateBookSchema},
    AppState,
};
use actix_web::{post, get, patch, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Hello world";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[post("/books/")]
async fn create_book_handler(body: web::Json<CreateBookSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        BookModel,
        "INSERT INTO Book (title, author_id) VALUES ($1, $2) RETURNING *",
        body.title.to_string(),
        body.author_id,
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(book) => {
            let response = serde_json::json!({"status": "success","data": serde_json::json!({
                "book": book
            })});

            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/books/{id}")]
async fn get_book_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let book_id = path.into_inner();
    let query_result = sqlx::query_as!(
        BookModel,
        "SELECT * FROM Book WHERE id = $1",
        book_id,
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(book) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "book": book
            })});

            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Book with ID: {} not found", book_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/books/{id}")]
async fn update_book_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateBookSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let book_id = path.into_inner();
    let query_result = sqlx::query_as!(
        BookModel,
        "UPDATE Book SET title = $1 WHERE id = $2 RETURNING *",
        body.title,
        book_id,
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(book) => {
            let response = serde_json::json!({"status": "success","data": serde_json::json!({
                "book": book
            })});

            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[delete("/books/{id}")]
async fn delete_book_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let book_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM Book WHERE id = $1", book_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();
    if rows_affected == 0 {
        let message = format!("Book with ID: {} not found", book_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }
    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check_handler)
        .service(create_book_handler)
        .service(get_book_handler)
        .service(update_book_handler)
        .service(delete_book_handler);

    conf.service(scope);
}