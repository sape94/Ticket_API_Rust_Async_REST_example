use axum::{
    routing::{get, patch, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

use ticket_api::{
    create_ticket, get_ticket, health_check, list_tickets, patch_ticket, AppState, TicketStore,
};

/// Entry point for the Ticket API server.
///
/// Sets up and runs the HTTP server with:
/// - Tracing for logging
/// - CORS middleware
/// - Route handlers for all endpoints
/// - Pretty-printed endpoint documentation
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::init();

    // Create the ticket store
    let store = TicketStore::new();

    // Build the application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tickets", post(create_ticket))
        .route("/tickets", get(list_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", patch(patch_ticket))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .with_state(store);

    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server starting on http://{}", addr);

    // Print available endpoints
    println!("📋 Available endpoints:");
    println!("  GET    /health           - Health check");
    println!("  POST   /tickets          - Create a new ticket");
    println!("  GET    /tickets          - List all tickets");
    println!("  GET    /tickets/:id      - Get a specific ticket");
    println!("  PATCH  /tickets/:id      - Update a specific ticket");
    println!();
    println!("📝 Example usage:");
    println!("  curl -X POST http://localhost:3000/tickets \\");
    println!("       -H 'Content-Type: application/json' \\");
    println!("       -d '{{\"title\":\"Fix bug\",\"description\":\"Fix the critical bug in the system\"}}'");
    println!();

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
