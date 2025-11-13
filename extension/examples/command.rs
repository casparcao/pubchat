use pubchat::{Extension, CommandHandler, CommandResult, extension::{ExtensionContext, ExtensionMethods}};
use anyhow::Result;

pub struct CommandExtension;

impl Extension for CommandExtension {
    fn name(&self) -> &str {
        "command"
    }

    fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        println!("Command Extension {} initialized with config: {:?}", self.name(), context.config);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        println!("Command extension shutdown!");
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CommandHandler for CommandExtension {
    fn commands(&self) -> Vec<&str> {
        vec!["time", "echo", "calc"]
    }

    fn handle_command(&self, command: &str, args: Vec<&str>) -> Result<CommandResult> {
        match command {
            "time" => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                Ok(CommandResult::Success(format!("Current timestamp: {}", now)))
            },
            "echo" => {
                Ok(CommandResult::Success(args.join(" ")))
            },
            "calc" => {
                if args.len() == 3 {
                    let a: f64 = args[0].parse().unwrap_or(0.0);
                    let op = args[1];
                    let b: f64 = args[2].parse().unwrap_or(0.0);
                    
                    let result = match op {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => {
                            if b != 0.0 {
                                a / b
                            } else {
                                return Ok(CommandResult::Error("Division by zero".to_string()));
                            }
                        },
                        _ => return Ok(CommandResult::Error("Unknown operator".to_string())),
                    };
                    
                    Ok(CommandResult::Success(format!("{} {} {} = {}", a, op, b, result)))
                } else {
                    Ok(CommandResult::Error("Usage: /calc <num1> <operator> <num2>".to_string()))
                }
            },
            _ => Ok(CommandResult::NotHandled),
        }
    }
}

fn main() {
    // This is just an example - actual usage would be in the main pubchat binary
    let mut extension = CommandExtension;
    let context = ExtensionContext {
        config: Default::default(),
        methods: ExtensionMethods::default(),
    };
    
    extension.initialize(&context).unwrap();
    println!("Extension name: {}", extension.name());
    extension.shutdown().unwrap();
}