//! Core data types and validation for the ticket management system.
//!
//! This module defines the fundamental data structures and their validation rules:
//! - Ticket components (ID, title, description, status)
//! - Request/response DTOs for the API
//! - Input validation logic
//! - Serialization/deserialization support

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a ticket.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TicketId(pub Uuid);

impl TicketId {
    /// Create a new random TicketId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Display implementation for TicketId.
impl fmt::Display for TicketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Title of a ticket. Must be non-empty and <= 100 chars.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketTitle(pub String);

impl TicketTitle {
    /// Validate and create a new TicketTitle.
    pub fn new(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 100 {
            return Err("Title cannot be longer than 100 characters".to_string());
        }
        Ok(Self(title))
    }
}

/// Description of a ticket. Must be <= 1000 chars.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketDescription(pub String);

impl TicketDescription {
    /// Validate and create a new TicketDescription.
    pub fn new(description: String) -> Result<Self, String> {
        if description.len() > 1000 {
            return Err("Description cannot be longer than 1000 characters".to_string());
        }
        Ok(Self(description))
    }
}

/// Represents a ticket in the system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

/// Draft for creating a new ticket.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

/// Status of a ticket.
/// Current status of a ticket.
///
/// Represents the workflow states a ticket can be in:
/// - `ToDo`: Work hasn't started
/// - `InProgress`: Work is currently being done
/// - `Done`: Work is completed
#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    /// Initial state for new tickets
    ToDo,
    /// Work has started on the ticket
    InProgress,
    /// Work is completed
    Done,
}

/// Display implementation for Status.
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::ToDo => write!(f, "To Do"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
}

/// Request payload for creating a ticket.
/// Request payload for creating a new ticket.
///
/// All fields are required and will be validated:
/// - `title`: Must be non-empty and <= 100 characters
/// - `description`: Must be <= 1000 characters
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    /// The ticket's title
    pub title: String,
    /// The ticket's description
    pub description: String,
}

/// Request payload for patching a ticket.
/// Request payload for updating an existing ticket.
///
/// All fields are optional. Only provided fields will be updated.
/// When provided, fields will be validated:
/// - `title`: Must be non-empty and <= 100 characters
/// - `description`: Must be <= 1000 characters
/// - `status`: Must be a valid Status enum value
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTicketRequest {
    /// Optional new title
    pub title: Option<String>,
    /// Optional new description
    pub description: Option<String>,
    /// Optional new status
    pub status: Option<Status>,
}

/// Response payload for a ticket.
/// Response payload representing a ticket.
///
/// This is the JSON format returned by the API for all ticket operations.
/// It flattens the internal ticket structure for a cleaner API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct TicketResponse {
    /// The ticket's unique identifier
    pub id: TicketId,
    /// The ticket's current title
    pub title: String,
    /// The ticket's current description
    pub description: String,
    /// The ticket's current status
    pub status: Status,
}

/// Convert Ticket to TicketResponse for API output.
impl From<Ticket> for TicketResponse {
    fn from(ticket: Ticket) -> Self {
        Self {
            id: ticket.id,
            title: ticket.title.0,
            description: ticket.description.0,
            status: ticket.status,
        }
    }
}
