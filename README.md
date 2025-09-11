# Ticket API - Rust Async REST Example

This project is an asynchronous REST API for managing tickets, built with Rust using the Axum web framework. It demonstrates modern Rust patterns for web development, including async/await, state management, error handling, and modular design.

## Features

- **Create Ticket**: Add new tickets with a title and description.
- **Retrieve Ticket**: Get details of a specific ticket by its ID.
- **Patch Ticket**: Update the title, description, or status of an existing ticket.
- **List Tickets**: Retrieve all tickets in the system.
- **Health Check**: Simple endpoint to verify the service is running.

## Documentation

- **[📚 Learning Guide](docs/learning-notebook.md)** - Comprehensive guide covering Rust async patterns, architecture analysis, and learning exercises
- **[🏗️ Architecture Overview](#architecture)** - High-level system design and module interactions

## Architecture

The application follows a layered architecture pattern:

![Module Architecture](docs/assets/architecture_diagram.png)

## Modules Overview

- `data.rs`: Defines core data structures for tickets, including types for ticket ID, title, description, status, and request/response payloads. Handles validation logic for input fields.
- `store.rs`: Implements an in-memory, thread-safe ticket store using `tokio::sync::RwLock` and `Arc`. Provides async methods to add, retrieve, patch, and list tickets. Custom error types for not found and invalid fields.
- `handlers.rs`: Contains Axum route handlers for each API endpoint. Handles request parsing, validation, error responses, and calls into the store.
- `lib.rs`: Re-exports modules for easy access and sets up the public API for the crate.
- `main.rs`: Sets up the Axum router, configures CORS, initializes tracing, and starts the HTTP server. Prints available endpoints and example usage.

## API Endpoints

| Method | Path           | Description              |
| ------ | -------------- | ------------------------ |
| GET    | `/health`      | Health check             |
| POST   | `/tickets`     | Create a new ticket      |
| GET    | `/tickets`     | List all tickets         |
| GET    | `/tickets/:id` | Get a specific ticket    |
| PATCH  | `/tickets/:id` | Update a specific ticket |

## Example Usage

Create a ticket:

```sh
curl -X POST http://localhost:3000/tickets \
     -H 'Content-Type: application/json' \
     -d '{"title":"Fix bug","description":"Fix the critical bug in the system"}'
```

## Running the Project

1. Install Rust (https://rustup.rs/)
2. Clone the repository
3. Run the server:
   ```sh
   cargo run
   ```
4. The API will be available at `http://localhost:3000`

## Dependencies

- [Axum](https://docs.rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization
- [Tower](https://github.com/tower-rs/tower) - Middleware
- [UUID](https://docs.rs/uuid) - Unique IDs
- [Tracing](https://docs.rs/tracing) - Logging

## License

**[MIT](LICENSE)** 
