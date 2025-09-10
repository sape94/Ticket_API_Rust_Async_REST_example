use crate::data::{PatchTicketRequest, Status, Ticket, TicketDraft, TicketId};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Errors that can occur in the ticket store.
/// Errors that can occur during ticket store operations.
#[derive(Debug, Error)]
pub enum StoreError {
    /// Returned when attempting to access a ticket that doesn't exist.
    #[error("Ticket with id {0} not found")]
    TicketNotFound(TicketId),

    /// Returned when a field validation fails during update.
    #[error("Invalid field: {0}")]
    InvalidField(String),
}

/// Thread-safe, in-memory store for tickets.
///
/// Uses a combination of [`Arc`] and [`RwLock`] to provide safe concurrent access
/// to tickets. Each ticket is individually locked to allow maximum concurrency
/// when modifying different tickets simultaneously.
#[derive(Clone)]
pub struct TicketStore {
    /// Inner storage using nested Arc and RwLock for fine-grained locking
    tickets: Arc<RwLock<HashMap<TicketId, Arc<RwLock<Ticket>>>>>,
}

impl TicketStore {
    /// Create a new, empty TicketStore.
    pub fn new() -> Self {
        Self {
            tickets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a new ticket from a draft. Returns the new ticket's ID.
    /// Adds a new ticket to the store from a draft.
    ///
    /// # Arguments
    /// * `draft` - The validated ticket draft containing title and description
    ///
    /// # Returns
    /// The ID of the newly created ticket
    pub async fn add_ticket(&self, draft: TicketDraft) -> TicketId {
        let id = TicketId::new();
        let ticket = Ticket {
            id: id.clone(),
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
        };

        let ticket_arc = Arc::new(RwLock::new(ticket));
        let mut tickets = self.tickets.write().await;
        tickets.insert(id.clone(), ticket_arc);

        id
    }

    /// Retrieve a ticket by its ID.
    /// Retrieves a ticket by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the ticket to retrieve
    ///
    /// # Returns
    /// * `Ok(Ticket)` - The requested ticket
    /// * `Err(StoreError::TicketNotFound)` - If no ticket exists with the given ID
    pub async fn get_ticket(&self, id: &TicketId) -> Result<Ticket, StoreError> {
        let tickets = self.tickets.read().await;
        match tickets.get(id) {
            Some(ticket_arc) => {
                let ticket = ticket_arc.read().await;
                Ok(ticket.clone())
            }
            None => Err(StoreError::TicketNotFound(id.clone())),
        }
    }

    /// Patch a ticket by its ID using the provided patch request.
    /// Updates specific fields of an existing ticket.
    ///
    /// # Arguments
    /// * `id` - The ID of the ticket to update
    /// * `patch` - The patch request containing optional updates to title, description, and status
    ///
    /// # Returns
    /// * `Ok(Ticket)` - The updated ticket
    /// * `Err(StoreError::TicketNotFound)` - If no ticket exists with the given ID
    /// * `Err(StoreError::InvalidField)` - If any of the updates fail validation
    pub async fn patch_ticket(
        &self,
        id: &TicketId,
        patch: PatchTicketRequest,
    ) -> Result<Ticket, StoreError> {
        let tickets = self.tickets.read().await;
        match tickets.get(id) {
            Some(ticket_arc) => {
                let mut ticket = ticket_arc.write().await;

                // Apply patches
                if let Some(title_str) = patch.title {
                    match crate::data::TicketTitle::new(title_str) {
                        Ok(title) => ticket.title = title,
                        Err(e) => return Err(StoreError::InvalidField(format!("title: {}", e))),
                    }
                }

                if let Some(description_str) = patch.description {
                    match crate::data::TicketDescription::new(description_str) {
                        Ok(description) => ticket.description = description,
                        Err(e) => {
                            return Err(StoreError::InvalidField(format!("description: {}", e)))
                        }
                    }
                }

                if let Some(status) = patch.status {
                    ticket.status = status;
                }

                Ok(ticket.clone())
            }
            None => Err(StoreError::TicketNotFound(id.clone())),
        }
    }

    /// List all tickets in the store.
    /// Retrieves all tickets from the store.
    ///
    /// # Returns
    /// A vector containing clones of all tickets currently in the store.
    /// Returns an empty vector if no tickets exist.
    pub async fn list_tickets(&self) -> Vec<Ticket> {
        let tickets = self.tickets.read().await;
        let mut result = Vec::new();

        for ticket_arc in tickets.values() {
            let ticket = ticket_arc.read().await;
            result.push(ticket.clone());
        }

        result
    }
}
