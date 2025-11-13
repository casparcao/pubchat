use pubchat::{Extension, ExtensionContext};
use anyhow::Result;

pub struct HelloWorldExtension;

impl Extension for HelloWorldExtension {
    fn name(&self) -> &str {
        "hello_world"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        println!("Hello, World! Extension {} initialized", self.name());
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        println!("Goodbye from {} extension!", self.name());
        Ok(())
    }
}

fn main() {
    // This is just an example - actual usage would be in the main pubchat binary
}