//! HTTP route handlers for the Ticket API.
//!
//! This module implements the REST API endpoints using Axum.
//! Each handler is responsible for:
//! - Validating incoming requests
//! - Interacting with the ticket store
//! - Formatting appropriate HTTP responses
//! - Error handling and status code selection
//!
//! The API supports the following operations:
//! - `POST /tickets` - Create a new ticket
//! - `GET /tickets` - List all tickets
//! - `GET /tickets/:id` - Get a specific ticket
//! - `PATCH /tickets/:id` - Update a ticket
//! - `GET /health` - Health check endpoint

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Json as RequestJson,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::data::{
    CreateTicketRequest, PatchTicketRequest, TicketDescription, TicketDraft, TicketId,
    TicketResponse, TicketTitle,
};
use crate::store::{StoreError, TicketStore};

/// Application state shared across all handlers.
/// Uses [`TicketStore`] for thread-safe ticket storage.
pub type AppState = TicketStore;

/// Creates a new ticket from the provided request payload.
///
/// # Request Body
/// Expects a JSON object with:
/// - `title`: String (1-100 characters)
/// - `description`: String (max 1000 characters)
///
/// # Returns
/// - `201 Created` with the created ticket on success
/// - `400 Bad Request` if validation fails
/// - `500 Internal Server Error` if ticket creation fails
pub async fn create_ticket(
    State(store): State<AppState>,
    RequestJson(request): RequestJson<CreateTicketRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Validate input
    let title = match TicketTitle::new(request.title) {
        Ok(title) => title,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid title",
                    "message": e
                })),
            ));
        }
    };

    let description = match TicketDescription::new(request.description) {
        Ok(description) => description,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid description",
                    "message": e
                })),
            ));
        }
    };

    let draft = TicketDraft { title, description };
    let ticket_id = store.add_ticket(draft).await;

    // Retrieve the created ticket to return complete information
    match store.get_ticket(&ticket_id).await {
        Ok(ticket) => {
            let response = TicketResponse::from(ticket);
            Ok((StatusCode::CREATED, Json(json!(response))))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create ticket"
            })),
        )),
    }
}

/// Retrieves a ticket by its UUID.
///
/// # Path Parameters
/// - `id`: UUID string of the ticket to retrieve
///
/// # Returns
/// - `200 OK` with the ticket data if found
/// - `400 Bad Request` if the UUID is invalid
/// - `404 Not Found` if no ticket matches the UUID
/// - `500 Internal Server Error` on unexpected errors
pub async fn get_ticket(
    State(store): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Parse UUID
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid ticket ID format"
                })),
            ));
        }
    };

    let ticket_id = TicketId(uuid);

    match store.get_ticket(&ticket_id).await {
        Ok(ticket) => {
            let response = TicketResponse::from(ticket);
            Ok(Json(json!(response)))
        }
        Err(StoreError::TicketNotFound(_)) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Ticket not found"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("Internal server error: {}", e)
            })),
        )),
    }
}

/// Updates specific fields of an existing ticket.
///
/// # Path Parameters
/// - `id`: UUID string of the ticket to update
///
/// # Request Body
/// JSON object with optional fields:
/// - `title`: Optional<String> (1-100 characters)
/// - `description`: Optional<String> (max 1000 characters)
/// - `status`: Optional<Status> ("ToDo", "InProgress", or "Done")
///
/// # Returns
/// - `200 OK` with the updated ticket
/// - `400 Bad Request` if validation fails or UUID is invalid
/// - `404 Not Found` if no ticket matches the UUID
pub async fn patch_ticket(
    State(store): State<AppState>,
    Path(id): Path<String>,
    RequestJson(patch_request): RequestJson<PatchTicketRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Parse UUID
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid ticket ID format"
                })),
            ));
        }
    };

    let ticket_id = TicketId(uuid);

    match store.patch_ticket(&ticket_id, patch_request).await {
        Ok(ticket) => {
            let response = TicketResponse::from(ticket);
            Ok(Json(json!(response)))
        }
        Err(StoreError::TicketNotFound(_)) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Ticket not found"
            })),
        )),
        Err(StoreError::InvalidField(msg)) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid field",
                "message": msg
            })),
        )),
    }
}

/// Lists all tickets in the system.
///
/// # Returns
/// - `200 OK` with an array of all tickets in the system
/// - Returns an empty array if no tickets exist
pub async fn list_tickets(State(store): State<AppState>) -> Json<Value> {
    let tickets = store.list_tickets().await;
    let responses: Vec<TicketResponse> = tickets.into_iter().map(TicketResponse::from).collect();
    Json(json!({
        "tickets": responses
    }))
}

/// Health check endpoint to verify the service is running.
///
/// # Returns
/// - `200 OK` with a JSON object containing:
///   - `status`: "healthy"
///   - `service`: "ticket-api"
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "ticket-api"
    }))
}
