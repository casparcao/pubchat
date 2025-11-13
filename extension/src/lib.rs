//! PubChat Extension System
//!
//! This crate provides the traits and types needed to create extensions for PubChat.
//! Extensions can hook into various parts of the system to add new functionality.

use std::collections::HashMap;
use anyhow::Result;

/// The main trait that all extensions must implement
pub trait Extension: Send + Sync {
    /// Get the name of the extension
    fn name(&self) -> &str;

    /// Initialize the extension with the given context
    fn initialize(&mut self, context: &ExtensionContext) -> Result<()>;

    /// Called when the extension is being shut down
    fn shutdown(&mut self) -> Result<()>;
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