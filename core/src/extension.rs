//! Extension manager for PubChat
//!
//! This module handles loading, initializing, and managing extensions.

use anyhow::Result;
use pubchat::extension::{Extension, ExtensionContext, LoadedExtension};
use std::collections::HashMap;

/// Manages extensions for the PubChat system
pub struct ExtensionManager {
    extensions: HashMap<String, LoadedExtension>,
}

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    /// Load an extension into the manager
    pub fn load_extension(&mut self, mut extension: Box<dyn Extension>) -> Result<()> {
        let name = extension.name().to_string();
        
        // Create context for the extension
        let context = ExtensionContext {
            config: HashMap::new(), // In a real implementation, this would be populated with actual config
            methods: Default::default(), // In a real implementation, this would contain actual methods
        };

        // Initialize the extension
        extension.initialize(&context)?;

        // Store the loaded extension
        self.extensions.insert(
            name.clone(),
            LoadedExtension {
                extension,
                enabled: true,
            },
        );

        tracing::info!("Loaded extension: {}", name);
        Ok(())
    }

    /// Get a reference to a loaded extension by name
    pub fn get_extension(&self, name: &str) -> Option<&LoadedExtension> {
        self.extensions.get(name)
    }

    /// Get a mutable reference to a loaded extension by name
    pub fn get_extension_mut(&mut self, name: &str) -> Option<&mut LoadedExtension> {
        self.extensions.get_mut(name)
    }

    /// List all loaded extensions
    pub fn list_extensions(&self) -> Vec<&String> {
        self.extensions.keys().collect()
    }

    /// Shutdown all extensions
    pub fn shutdown(&mut self) -> Result<()> {
        for (name, loaded_ext) in &mut self.extensions {
            if let Err(e) = loaded_ext.extension.shutdown() {
                tracing::error!("Error shutting down extension {}: {}", name, e);
            }
        }
        Ok(())
    }
}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}