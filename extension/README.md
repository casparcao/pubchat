# PubChat Extension System

This crate provides the foundation for extending PubChat functionality through plugins/extensions.

## Overview

Extensions in PubChat are Rust crates that implement the [Extension](src/lib.rs) trait. They can hook into various parts of the system to add new functionality.

## Creating an Extension

To create an extension:

1. Create a new Rust library crate
2. Add `pubchat` as a dependency
3. Implement the [Extension](src/lib.rs) trait
4. Optionally implement additional traits like [MessageProcessor](src/extension/mod.rs) or [CommandHandler](src/extension/mod.rs)
5. Build and package your extension

### Example Extension

```rust
use pubchat::{Extension, MessageProcessor, CommandHandler, CommandResult, extension::{ExtensionContext, ExtensionMethods}};
use anyhow::Result;
use core::proto::message::Message;

pub struct MyExtension;

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "my_extension"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        // Initialize your extension here
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        // Clean up your extension here
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl MessageProcessor for MyExtension {
    fn on_message_receive(&self, message: &mut Message) -> Result<bool> {
        // Process incoming messages
        Ok(true) // Return false to block the message
    }

    fn on_message_send(&self, message: &mut Message) -> Result<bool> {
        // Process outgoing messages
        Ok(true) // Return false to block the message
    }
}

impl CommandHandler for MyExtension {
    fn commands(&self) -> Vec<&str> {
        vec!["mycommand", "another"]
    }

    fn handle_command(&self, command: &str, args: Vec<&str>) -> Result<CommandResult> {
        match command {
            "mycommand" => Ok(CommandResult::Success("Handled mycommand!".to_string())),
            _ => Ok(CommandResult::NotHandled),
        }
    }
}
```

## Extension Types

### MessageProcessor

Implement the [MessageProcessor](src/extension/mod.rs) trait to intercept and process messages:

- `on_message_receive`: Process incoming messages before they are handled
- `on_message_send`: Process outgoing messages before they are sent

Return `false` from these methods to block the message, or `true` to allow it to continue processing.

### CommandHandler

Implement the [CommandHandler](src/extension/mod.rs) trait to add new commands:

- `commands`: Return a list of commands this extension handles
- `handle_command`: Process a command and return a result

Return [CommandResult::NotHandled](src/extension/mod.rs) if the command is not recognized by this handler.

## Extension Lifecycle

1. **Loading**: The extension is loaded by the ExtensionManager
2. **Initialization**: The `initialize` method is called with an [ExtensionContext](src/extension/mod.rs)
3. **Runtime**: The extension is active and can interact with the system
4. **Shutdown**: The `shutdown` method is called when the system shuts down

## Extension Context

The [ExtensionContext](src/extension/mod.rs) provides extensions with:

- Configuration values
- Methods to interact with the core system

Currently, these are placeholders and will be expanded in future versions.

## Examples

See the [examples](examples/) directory for sample extensions:

- [hello_world.rs](examples/hello_world.rs): Basic extension with message processing and command handling
- [filter.rs](examples/filter.rs): Message filtering extension
- [command.rs](examples/command.rs): Command handling extension