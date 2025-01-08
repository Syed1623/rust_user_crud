# Rust CRUD API with Rocket and Diesel

This project is a RESTful API built with Rust, using Rocket as the web framework and Diesel as the ORM with PostgreSQL database.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL (12 or higher)
- Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)

## Project Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-api-crud
```

2. Create a `.env` file in the project root:
```bash
echo DATABASE_URL=postgres://postgres:password@localhost/rust_api_crud > .env
```
Replace `postgres`, `password` with your PostgreSQL credentials.

3. Create the database:
```bash
psql -U postgres
CREATE DATABASE rust_api_crud;
```

4. Run the migrations:
```bash
diesel setup
diesel migration run
```

5. Build and run the project:
```bash
cargo build
cargo run
```

The server will start at `http://localhost:8000`

## API Endpoints

### Get all users
```bash
curl http://localhost:8000/api/users
```

### Get user by ID
```bash
curl http://localhost:8000/api/users/1
```

### Create new user
```bash
curl -X POST -H "Content-Type: application/json" -d '{"email":"user@example.com","name":"John Doe"}' http://localhost:8000/api/users
```

### Update user
```bash
curl -X PUT -H "Content-Type: application/json" -d '{"email":"updated@example.com","name":"Jane Doe"}' http://localhost:8000/api/users/1
```

### Delete user
```bash
curl -X DELETE http://localhost:8000/api/users/1
```

## Running Tests

To run the tests:
```bash
cargo test
```

## Project Structure

```
├── Cargo.toml          # Project dependencies
├── Rocket.toml         # Rocket configuration
├── .env                # Environment variables
├── migrations/         # Database migrations
│   └── *_create_users/
│       ├── up.sql     # Create table
│       └── down.sql   # Drop table
└── src/
    ├── main.rs        # Application entry point
    ├── db.rs          # Database connection
    ├── schema.rs      # Database schema
    ├── models/        # Data models
    │   ├── mod.rs
    │   └── user.rs
    ├── repositories/  # Data access
    │   ├── mod.rs
    │   └── user_repository.rs
    └── routes/        # API endpoints
        ├── mod.rs
        └── user_routes.rs
```

## Sample .env File

Create a `.env` file in the project root with the following content:
```env
DATABASE_URL=postgres://username:password@localhost/rust_api_crud
ROCKET_ADDRESS=127.0.0.1
ROCKET_PORT=8000
```

## Testing with sample data

Here's a complete test sequence using curl:

1. Create a new user:
```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"email":"john@example.com","name":"John Doe"}' \
  http://localhost:8000/api/users
```

2. List all users:
```bash
curl http://localhost:8000/api/users
```

3. Get specific user (replace 1 with the actual ID):
```bash
curl http://localhost:8000/api/users/1
```

4. Update user:
```bash
curl -X PUT -H "Content-Type: application/json" \
  -d '{"email":"john.updated@example.com","name":"John Updated"}' \
  http://localhost:8000/api/users/1
```

5. Delete user:
```bash
curl -X DELETE http://localhost:8000/api/users/1
```

## Common Issues and Solutions

1. Database Connection Issues:
   - Verify PostgreSQL is running
   - Check credentials in `.env` file
   - Ensure database exists

2. Migration Issues:
   - Run `diesel migration revert` to undo migrations
   - Run `diesel migration run` to apply migrations
   - Check `schema.rs` is up to date

3. Compilation Errors:
   - Run `cargo clean` and then `cargo build`
   - Verify all dependencies in `Cargo.toml`
   - Check for missing features in dependencies

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/awesome-feature`)
3. Commit your changes (`git commit -am 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome-feature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details