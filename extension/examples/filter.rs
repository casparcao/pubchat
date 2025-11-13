use pubchat::extension::{Extension, MessageProcessor, ExtensionContext, ExtensionMethods};
use anyhow::Result;
use pubchat::core::message::Message;

pub struct FilterExtension;

impl Extension for FilterExtension {
    fn name(&self) -> &str {
        "filter"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        println!("Filter Extension {} initialized with config: {:?}", self.name(), context.config);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        println!("Filter extension shutdown!");
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl MessageProcessor for FilterExtension {
    fn on_message_receive(&self, message: &mut Message) -> Result<bool> {
        println!("Filtering incoming message: {:?}", message);
        // For demo purposes, let's block messages containing "blockme"
        if let Some(content) = &message.content {
            match content {
                pubchat::core::message::message::Content::Chrt(chrt) => {
                    if let Some(msg) = &chrt.message {
                        match msg {
                            pubchat::core::message::chrt::Message::Text(text) => {
                                if text.text.contains("blockme") {
                                    println!("Blocking message containing 'blockme'");
                                    return Ok(false); // Block the message
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        // Allow all other messages
        Ok(true)
    }

    fn on_message_send(&self, message: &mut Message) -> Result<bool> {
        println!("Filtering outgoing message: {:?}", message);
        // For demo purposes, let's modify messages containing "replace_me"
        if let Some(content) = &mut message.content {
            match content {
                pubchat::core::message::message::Content::Chrt(chrt) => {
                    if let Some(msg) = &mut chrt.message {
                        match msg {
                            pubchat::core::message::chrt::Message::Text(text) => {
                                if text.text.contains("replace_me") {
                                    text.text = text.text.replace("replace_me", "REPLACED");
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        // Allow all messages
        Ok(true)
    }
}

fn main() {
    // This is just an example - actual usage would be in the main pubchat binary
    let mut extension = FilterExtension;
    let context = ExtensionContext {
        config: Default::default(),
        methods: ExtensionMethods::default(),
    };
    
    extension.initialize(&context).unwrap();
    println!("Extension name: {}", extension.name());
    extension.shutdown().unwrap();
}