use pubchat::extension::{CommandExtension, CommandResult, Extension, ExtensionContext, ExtensionMethods, MessageExtension};
use anyhow::Result;
use pubchat::core::message::Message;

pub struct HelloWorldExtension;

impl Extension for HelloWorldExtension {
    fn name(&self) -> &str {
        "hello_world"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        println!("Hello, World! Extension {} initialized with config: {:?}", self.name(), context.config);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        println!("Goodbye from {} extension!", self.name());
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl MessageExtension for HelloWorldExtension {
    fn on_message_receive(&self, message: &mut Message) -> Result<bool> {
        println!("Processing incoming message: {:?}", message);
        // Always allow the message to continue processing
        Ok(true)
    }

    fn on_message_send(&self, message: &mut Message) -> Result<bool> {
        println!("Processing outgoing message: {:?}", message);
        // Always allow the message to be sent
        Ok(true)
    }
}

/// usage: !hello world
impl CommandExtension for HelloWorldExtension {

    fn command(&self) -> &str {
        "world"
    }

    fn help(&self) -> &str {
        "Say hello to someone or to the world"
    }

    fn prefix(&self) -> &str {
        "hello"
    }

    fn execute(&self, args: Vec<&str>) -> Result<CommandResult> {
        Ok(CommandResult::Success("World says hello back!".to_string()))
    }
}

fn main() {
    // This is just an example - actual usage would be in the main pubchat binary
    let mut extension = HelloWorldExtension;
    let context = ExtensionContext {
        config: Default::default(),
        methods: ExtensionMethods::default(),
    };
    
    extension.initialize(&context).unwrap();
    println!("Extension name: {}", extension.name());
    extension.shutdown().unwrap();
}