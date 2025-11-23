# PubChat Extension System

The extension system allows developers to extend the functionality of PubChat without modifying the core codebase.

## Overview

Extensions can hook into various parts of the application lifecycle and customize behavior such as message processing, command handling, and more.

## Core Concepts

### Extension Trait

All extensions must implement the `Extension` trait which defines the basic extension interface:

```rust
pub trait Extension: Send + Sync {
    fn name(&self) -> &str;
    fn initialize(&mut self, context: &ExtensionContext) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    fn as_any(&self) -> &dyn std::any::Any;
}
```

### Specialized Extension Traits

Extensions can also implement specialized traits for specific functionality:

1. `MessageExtension` - For processing messages
2. `CommandExtension` - For handling commands

## Creating an Extension

To create an extension, implement the `Extension` trait and any additional extension traits you need:

```rust
use pubchat::extension::{Extension, ExtensionContext, MessageExtension, CommandExtension};
use anyhow::Result;
use pubchat::core::message::Message;

pub struct MyExtension;

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "MyExtension"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        // Initialization code
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        // Cleanup code
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl MessageExtension for MyExtension {
    fn on_message_receive(&self, message: &mut Message) -> Result<bool> {
        // Process incoming messages
        Ok(true) // Allow message to continue processing
    }

    fn on_message_send(&self, message: &mut Message) -> Result<bool> {
        // Process outgoing messages
        Ok(true) // Allow message to be sent
    }
}

impl CommandExtension for MyExtension {
    fn command(&self) -> &str {
        "mycommand"
    }

    fn help(&self) -> &str {
        "Description of what mycommand does"
    }

    fn prefix(&self) -> &str {
        "myext"
    }

    fn execute(&self, args: Vec<&str>) -> Result<CommandResult> {
        Ok(CommandResult::Success("Command executed successfully".to_string()))
    }
}
```

## Plugin Package Format (.pubchat)

PubChat supports plugin packages with the .pubchat extension. These are ZIP files with a specific structure:

```
example_plugin.pubchat (ZIP file)
├── manifest.json
├── plugin.dll (or .so, .dylib - compiled plugin library)
└── [other assets]
```

### Manifest Format

The manifest.json file contains metadata about the plugin:

```json
{
  "name": "example_plugin",
  "version": "1.0.0",
  "description": "An example plugin for PubChat",
  "main": "plugin.dll",
  "author": "Your Name",
  "repository": "https://github.com/user/repo",
  "display_name": "Example Plugin",
  "categories": ["example", "demo"],
  "keywords": ["example", "demo"],
  "license": "MIT",
  "homepage": "https://example.com"
}
```

### Building a Plugin Package

To build a plugin package, create a ZIP file with the manifest and compiled library:

```rust
use std::fs::File;
use std::io::{Write, BufWriter};
use zip::{ZipWriter, CompressionMethod};

// Create the plugin package (.pubchat file)
let package_file = File::create("example_plugin.pubchat")?;
let mut zip = ZipWriter::new(BufWriter::new(package_file));

// Add manifest.json
zip.start_file("manifest.json", CompressionMethod::Deflated)?;
// ... write manifest content

// Add plugin library
zip.start_file("plugin.dll", CompressionMethod::Deflated)?;
// ... write library content

zip.finish()?;
```

## Complete Example Plugin

There is a complete example plugin in the `example` directory that demonstrates all features of the extension system:

1. Message processing
2. Command handling
3. Proper packaging for distribution

To build the complete example plugin package:

```bash
cd example
cargo run --bin build_complete_package
```

This will create a `complete_example.pubchat` file that can be placed in the client's plugins directory.

## Loading Extensions

### Static Loading

Extensions can be statically loaded by registering them directly with the ExtensionManager:

```rust
let mut extension_manager = ExtensionManager::new();
extension_manager.load_extension(Box::new(MyExtension));
```

### Package-based Loading

Extensions can be loaded from .pubchat package files:

1. Place plugin packages in the `plugins` directory
2. The PubChat client will automatically load all .pubchat files in this directory at startup

The client looks for functions with the signature `fn _create_extension() -> Box<dyn Extension>` in the plugin library.

## Built-in Extension Examples

The extension crate includes several example extensions:

1. `hello_world.rs` - Basic extension with message processing and command handling
2. `filter.rs` - Message filtering extension
3. `command.rs` - Command handling extension
4. `build_package.rs` - Example of how to build a plugin package
5. `complete_plugin.rs` - A complete example showing all features

## Extension Lifecycle

1. **Loading**: The extension is loaded by the ExtensionManager
2. **Initialization**: The `initialize` method is called with an ExtensionContext
3. **Runtime**: The extension is active and can interact with the system
4. **Shutdown**: The `shutdown` method is called when the system shuts down

## Extension Context

The ExtensionContext provides extensions with:

- Configuration values
- Methods to interact with the core system

Currently, these are placeholders and will be expanded in future versions.

## Command Handling

Extensions can handle commands in two ways:

1. **Direct plugin commands**: Using the `!plugin.command` format (e.g., `!hello.world`)
2. **Global command handlers**: Extensions register command handlers that are tried in sequence

The direct plugin format is preferred as it's more efficient and avoids naming conflicts.

## Future Improvements

Planned improvements to the extension system include:

- Plugin configuration via files
- Plugin communication mechanisms
- Performance metrics for extensions
- Plugin permissions and sandboxing
- Hot-reloading of extensions during development
- Plugin marketplace/repository
- Plugin signature verification for security