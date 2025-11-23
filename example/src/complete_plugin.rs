//! A complete example plugin that demonstrates all the features of the PubChat extension system.
//!
//! This plugin can be compiled into a dynamic library and packaged into a .pubchat file.

use pubchat::extension::{
    CommandExtension, 
    CommandResult, 
    Extension, 
    ExtensionContext, 
    MessageExtension
};
use anyhow::Result;
use pubchat::core::message::Message;

/// A complete example plugin
pub struct CompleteExamplePlugin;

impl Extension for CompleteExamplePlugin {
    fn name(&self) -> &str {
        "complete_example"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        println!("Complete Example Plugin initialized with config: {:?}", context.config);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        println!("Complete Example Plugin shutting down");
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl MessageExtension for CompleteExamplePlugin {
    fn on_message_receive(&self, message: &mut Message) -> Result<bool> {
        // Process incoming messages
        println!("[Complete Plugin] Special message received: {:?}", message.content);
        Ok(true) // Allow message to continue processing
    }

    fn on_message_send(&self, message: &mut Message) -> Result<bool> {
        // Process outgoing messages
        println!("[Complete Plugin] Special message being sent: {:?}", message.content);
        Ok(true) // Allow message to be sent
    }
}

impl CommandExtension for CompleteExamplePlugin {
    fn command(&self) -> &str {
        "complete"
    }

    fn help(&self) -> &str {
        "A complete example plugin command"
    }

    fn prefix(&self) -> &str {
        "example"
    }

    fn execute(&self, args: Vec<&str>) -> Result<CommandResult> {
        if args.is_empty() {
            Ok(CommandResult::Success("Complete plugin is working!".to_string()))
        } else {
            Ok(CommandResult::Success(format!("Complete plugin received args: {:?}", args)))
        }
    }
}
