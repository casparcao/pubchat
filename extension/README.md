# PubChat Extension System

This crate provides the foundation for extending PubChat functionality through plugins/extensions.

## Overview

Extensions in PubChat are Rust crates that implement the [Extension](src/lib.rs) trait. They can hook into various parts of the system to add new functionality.

## Creating an Extension

To create an extension:

1. Create a new Rust library crate
2. Add `pubchat-extension` as a dependency
3. Implement the [Extension](src/lib.rs) trait
4. Build and package your extension

### Example Extension

```rust
use pubchat_extension::{Extension, ExtensionContext};
use anyhow::Result;

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
}
```

## Extension Lifecycle

1. **Loading**: The extension is loaded by the ExtensionManager
2. **Initialization**: The `initialize` method is called with an [ExtensionContext](src/lib.rs)
3. **Runtime**: The extension is active and can interact with the system
4. **Shutdown**: The `shutdown` method is called when the system shuts down

## Extension Context

The [ExtensionContext](src/lib.rs) provides extensions with:

- Configuration values
- Methods to interact with the core system

Currently, these are placeholders and will be expanded in future versions.