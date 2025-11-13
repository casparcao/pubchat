//! PubChat Extension System
//!
//! This crate provides the traits and types needed to create extensions for PubChat.
//! Extensions can hook into various parts of the system to add new functionality.

use std::collections::HashMap;
use anyhow::Result;
use crate::core::message::Message;

/// The main trait that all extensions must implement
pub trait Extension: Send + Sync {
    /// Get the name of the extension
    fn name(&self) -> &str;

    /// Initialize the extension with the given context
    fn initialize(&mut self, context: &ExtensionContext) -> Result<()>;

    /// Called when the extension is being shut down
    fn shutdown(&mut self) -> Result<()>;
    
    /// Allow downcasting to concrete types
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Context provided to extensions during initialization
pub struct ExtensionContext {
    /// Configuration values for the extension
    pub config: HashMap<String, String>,
    
    /// Methods to interact with the core system
    pub methods: ExtensionMethods,
}

impl Default for ExtensionContext {
    fn default() -> Self {
        Self {
            config: HashMap::new(),
            methods: ExtensionMethods::default(),
        }
    }
}

/// Methods that extensions can use to interact with the core system
#[derive(Default)]
pub struct ExtensionMethods {
    // TODO: Add methods for extensions to interact with the core system
    // For example:
    // - Register new API endpoints
    // - Subscribe to message events
    // - Access user/session data (with permissions)
    // - Send messages
}

/// A loaded extension with its metadata
pub struct LoadedExtension {
    /// The extension instance
    pub extension: Box<dyn Extension>,
    
    /// Whether the extension is currently enabled
    pub enabled: bool,
}

/// Trait for message processing extensions
/// 
/// This trait allows plugins to intercept and process messages at various stages
pub trait MessageProcessor: Extension {
    /// Process an incoming message before it's handled by the system
    /// 
    /// This method is called when a message is received but before it's processed.
    /// The extension can modify, filter, or block the message.
    /// 
    /// Return `true` to allow the message to continue processing, or `false` to stop processing
    fn on_message_receive(&self, message: &mut Message) -> Result<bool> {
        // Default implementation does nothing and allows the message
        let _ = message;
        Ok(true)
    }

    /// Process an outgoing message before it's sent
    /// 
    /// This method is called when a message is about to be sent.
    /// The extension can modify or block the message.
    /// 
    /// Return `true` to allow the message to be sent, or `false` to prevent sending
    fn on_message_send(&self, message: &mut Message) -> Result<bool> {
        // Default implementation does nothing and allows the message
        let _ = message;
        Ok(true)
    }
}

/// Trait for command extensions
/// 
/// This trait allows plugins to register and handle custom commands
pub trait CommandHandler: Extension {
    /// Get a list of commands this extension handles
    fn commands(&self) -> Vec<&str>;

    /// Handle a command
    /// 
    /// This method is called when a user enters a command that this extension has registered
    fn handle(&self, command: &str, args: Vec<&str>) -> Result<CommandResult>;
}

/// Result of command execution
pub enum CommandResult {
    /// Command was handled successfully
    Success(String),
    /// Command was handled but with an error
    Error(String),
    /// Command was not recognized by this handler
    Ignore,
}

impl<T: Extension + 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Add a trait to allow downcasting
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl AsRef<dyn Extension> for dyn MessageProcessor {
    fn as_ref(&self) -> &(dyn Extension + 'static) {
        self
    }
}

impl AsRef<dyn Extension> for dyn CommandHandler {
    fn as_ref(&self) -> &(dyn Extension + 'static) {
        self
    }
}