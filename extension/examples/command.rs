use pubchat::extension::{Extension, CommandExtension, CommandResult, ExtensionContext, ExtensionMethods};
use anyhow::Result;

pub struct CalcCommandExtension;

impl Extension for CalcCommandExtension {
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

impl CommandExtension for CalcCommandExtension {
    fn command(&self) -> &str {
        "calc"
    }

    fn execute(&self, args: Vec<&str>) -> Result<CommandResult> {
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
    }

    fn help(&self) -> &str {
         "!demo calc <num1> <operator> <num2> - Calculate the result of the operation"
    }

    fn prefix(&self) -> &str {
        "demo"
    }

}

fn main() {
    // This is just an example - actual usage would be in the main pubchat binary
    let mut extension = CalcCommandExtension;
    let context = ExtensionContext {
        config: Default::default(),
        methods: ExtensionMethods::default(),
    };
    
    extension.initialize(&context).unwrap();
    println!("Extension name: {}", extension.name());
    extension.shutdown().unwrap();
}