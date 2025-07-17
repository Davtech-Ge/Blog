# Fullstack Blog API

This project is a fullstack blog backend built with Rust, Actix-web, and Diesel ORM (SQLite). It provides RESTful endpoints for managing users, posts, and comments.

## Features
- User registration and lookup (by username or ID)
- Post creation, publishing, and listing
- Comment creation and retrieval
- Error handling with custom `AppError` type
- SQLite database with Diesel migrations

## Endpoints
### Users
- `POST /users` — Create a new user
- `GET /users/find/{name}` — Find user by username
- `GET /users/find/id/{user_id}` — Find user by ID

### Posts
- `POST /posts/{user_id}` — Create a post for a user
- `GET /posts` — List all published posts
- `POST /posts/publish/{post_id}` — Publish a post

### Comments
- `POST /comments/{post_id}` — Add a comment to a post
- `GET /comments/{post_id}` — Get all comments for a post

## Setup
1. Install Rust, Diesel CLI, and SQLite.
2. Clone the repo and run migrations:
   ```bash
   diesel setup
   diesel migration run
   ```
3. Start the server:
   ```bash
   cargo run
   ```

## Example Usage
```bash
# Create a user
curl -X POST -H 'Content-Type: application/json' \
     -d '{"username": "Alice"}' \
     http://localhost:8998/users

# Find user by name
curl http://localhost:8998/users/find/Alice

# Create a post
curl -X POST -H 'Content-Type: application/json' \
     -d '{"title": "My Post", "body": "Hello world!"}' \
     http://localhost:8998/posts/1
```

## License
MIT
