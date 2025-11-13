//! Extension manager for PubChat
//!
//! This module handles loading, initializing, and managing extensions.

use anyhow::Result;
use pubchat::extension::{Extension, ExtensionContext, LoadedExtension, MessageProcessor, CommandHandler};
use std::collections::HashMap;
use crate::proto::message::Message;

/// Manages extensions for the PubChat system
pub struct ExtensionManager {
    extensions: HashMap<String, LoadedExtension>,
    message_processors: Vec<String>, // Store extension names that implement MessageProcessor
    command_handlers: Vec<String>,   // Store extension names that implement CommandHandler
}

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            message_processors: Vec::new(),
            command_handlers: Vec::new(),
        }
    }

    /// Load an extension into the manager
    pub fn load_extension(&mut self, mut extension: Box<dyn Extension>) -> Result<()> {
        let name = extension.name().to_string();
        
        // Check if extension implements MessageProcessor
        let is_message_processor = extension.as_any().is::<dyn MessageProcessor>();
        if is_message_processor {
            self.message_processors.push(name.clone());
        }
        
        // Check if extension implements CommandHandler
        let is_command_handler = extension.as_any().is::<dyn CommandHandler>();
        if is_command_handler {
            self.command_handlers.push(name.clone());
        }
        
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

    /// Process an incoming message through all registered message processors
    pub fn process_incoming_message(&self, message: &mut Message) -> Result<bool> {
        for processor_name in &self.message_processors {
            if let Some(loaded_ext) = self.extensions.get(processor_name) {
                // Try to downcast to MessageProcessor
                if let Some(processor) = loaded_ext.extension.as_any().downcast_ref::<dyn MessageProcessor>() {
                    if !processor.on_message_receive(message)? {
                        // If any processor returns false, stop processing
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

    /// Process an outgoing message through all registered message processors
    pub fn process_outgoing_message(&self, message: &mut Message) -> Result<bool> {
        for processor_name in &self.message_processors {
            if let Some(loaded_ext) = self.extensions.get(processor_name) {
                // Try to downcast to MessageProcessor
                if let Some(processor) = loaded_ext.extension.as_any().downcast_ref::<dyn MessageProcessor>() {
                    if !processor.on_message_send(message)? {
                        // If any processor returns false, stop processing
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

    /// Handle a command through all registered command handlers
    pub fn handle_command(&self, command: &str, args: Vec<&str>) -> Result<Option<pubchat::extension::CommandResult>> {
        for handler_name in &self.command_handlers {
            if let Some(loaded_ext) = self.extensions.get(handler_name) {
                // Try to downcast to CommandHandler
                if let Some(handler) = loaded_ext.extension.as_any().downcast_ref::<dyn CommandHandler>() {
                    match handler.handle_command(command, args.clone())? {
                        pubchat::extension::CommandResult::NotHandled => continue,
                        result => return Ok(Some(result)),
                    }
                }
            }
        }
        Ok(None)
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

// Add a trait to allow downcasting
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: Extension> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}