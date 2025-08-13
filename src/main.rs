use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

// ------------------ Type Alias ------------------
type Db = Pool<Postgres>;

// ------------------ Structs ------------------
#[derive(Deserialize, Serialize, FromRow)]
struct Book {
    id: Uuid,
    title: String,
    author: String,
    isbn: String,
    genre: String,
    quantity: i32,
    available: i32,
}

#[derive(Deserialize)]
struct NewBook {
    title: String,
    author: String,
    isbn: String,
    genre: String,
    quantity: i32,
    available: i32,
}

// ------------------ DB Connection ------------------
async fn connect_to_db() -> Db {
    dotenv().ok();
    let database_url = env::var("SUPABASE_URL")
        .expect("Please put a proper SUPABASE_URL in .env");
    Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Cant connect to the database url")
}

// ------------------ CRUD Routes ------------------

// Add a new book
#[post("/books")]
async fn add_book(pool: web::Data<Db>, book: web::Json<NewBook>) -> impl Responder {
    let id = Uuid::new_v4();

    let query = sqlx::query!(
        r#"
        INSERT INTO books (id, title, author, isbn, genre, quantity, available)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        id,
        book.title,
        book.author,
        book.isbn,
        book.genre,
        book.quantity,
        book.available
    )
    .execute(pool.get_ref())
    .await;

    match query {
        Ok(_) => HttpResponse::Ok().json(format!("Book {} added successfully", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// List all books
#[get("/books")]
async fn list_books(pool: web::Data<Db>) -> impl Responder {
    let rows = sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(pool.get_ref())
        .await;

    match rows {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Update a book
#[put("/books/{id}")]
async fn update_book(
    pool: web::Data<Db>,
    book_id: web::Path<Uuid>,
    updated: web::Json<NewBook>,
) -> impl Responder {
    let query = sqlx::query!(
        r#"
        UPDATE books SET title=$1, author=$2, isbn=$3, genre=$4, quantity=$5, available=$6
        WHERE id=$7
        "#,
        updated.title,
        updated.author,
        updated.isbn,
        updated.genre,
        updated.quantity,
        updated.available,
        *book_id
    )
    .execute(pool.get_ref())
    .await;

    match query {
        Ok(_) => HttpResponse::Ok().json("Book updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Delete a book
#[delete("/books/{id}")]
async fn delete_book(pool: web::Data<Db>, book_id: web::Path<Uuid>) -> impl Responder {
    let query = sqlx::query!("DELETE FROM books WHERE id=$1", *book_id)
        .execute(pool.get_ref())
        .await;

    match query {
        Ok(_) => HttpResponse::Ok().json("Book deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// ------------------ Main ------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connect_to_db().await;

    println!("🚀 Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_book)
            .service(list_books)
            .service(update_book)
            .service(delete_book)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
