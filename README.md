# 📚 Library Management System (Rust + Actix + Supabase)

A simple **Library Management System backend** built in **Rust** using **Actix Web** and **Supabase (PostgreSQL)**.  
Supports **Books CRUD operations** (Create, Read, Update, Delete).

---

## Features

- Add a new book  
- List all books  
- Update a book  
- Delete a book  
- Fully asynchronous and fast using **Actix Web**  
- Uses **UUIDs** for unique book IDs  
- Database: **Supabase Postgres**  
- Compile-time checked SQL with **SQLx**  

---

## Tech Stack

| Layer        | Technology             |
|-------------|----------------------|
| Backend     | Rust + Actix Web      |
| Database    | Supabase (Postgres)   |
| ORM / SQL  | SQLx                  |
| UUID        | uuid crate            |
| Environment | dotenvy crate         |

---

## Getting Started

### 1️⃣ Clone the repository
```bash
git clone <your-repo-url>
cd library-backend
2️⃣ Setup .env
Create a .env file in the root directory:

env
Copy
Edit
SUPABASE_URL=postgres://<your-supabase-db-url>
3️⃣ Install Rust dependencies
bash
Copy
Edit
cargo build
cargo run
4️⃣ Create Database Table
Make sure your Supabase database has a books table:

sql
Copy
Edit
CREATE TABLE books (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    isbn TEXT,
    genre TEXT,
    quantity INT NOT NULL,
    available INT NOT NULL
);
API Endpoints
Method	Endpoint	Description	Body (JSON) Example
POST	/books	Add a new book	{ "title": "Rust Book", "author": "Steve", "isbn": "1234", "genre": "Programming", "quantity": 5, "available": 5 }
GET	/books	List all books	-
PUT	/books/{id}	Update a book by ID	{ "title": "New Title", "author": "New Author", "isbn": "5678", "genre": "Programming", "quantity": 10, "available": 8 }
DELETE	/books/{id}	Delete a book by ID	-
