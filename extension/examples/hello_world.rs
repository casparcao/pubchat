use pubchat::extension::{CommandHandler, CommandResult, Extension, ExtensionContext, ExtensionMethods, MessageProcessor};
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

impl MessageProcessor for HelloWorldExtension {
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

impl CommandHandler for HelloWorldExtension {
    fn commands(&self) -> Vec<&str> {
        vec!["hello", "world"]
    }

    fn handle(&self, command: &str, args: Vec<&str>) -> Result<CommandResult> {
        match command {
            "hello" => Ok(CommandResult::Success(format!("Hello, {}!", args.get(0).unwrap_or(&"World")))),
            "world" => Ok(CommandResult::Success("World says hello back!".to_string())),
            _ => Ok(CommandResult::Ignore),
        }
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