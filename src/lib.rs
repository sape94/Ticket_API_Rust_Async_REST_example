//! Ticket API - An asynchronous REST API for ticket management.
//!
//! This crate provides a complete solution for managing tickets through a REST API,
//! with features including:
//! - Creating, retrieving, and updating tickets
//! - Input validation and error handling
//! - Thread-safe in-memory storage
//! - CORS support and health checking
//!
//! The crate is organized into three main modules:
//! - `data`: Core data types and validation
//! - `handlers`: HTTP route handlers
//! - `store`: Thread-safe ticket storage

/// Core data structures and validation logic for the ticket system.
/// Includes types for tickets, their components, and request/response DTOs.
pub mod data;

/// HTTP route handlers implementing the REST API endpoints.
/// Uses Axum for routing and request handling.
pub mod handlers;

/// Thread-safe, in-memory storage for tickets.
/// Provides CRUD operations with proper error handling.
pub mod store;

pub use data::*;
pub use handlers::*;
pub use store::*;
