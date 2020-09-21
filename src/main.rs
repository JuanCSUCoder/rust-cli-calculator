use std::io;
use io::Write;

// This prints a welcome message
fn welcome(){
    println!("Welcome to this CLI Rust Calculator");
    println!("Write down the expression to solve or type 0 to exit");
}

fn has_operations(strexp: String) -> bool {
    if strexp.contains("+") || strexp.contains("-") || strexp.contains("*") || strexp.contains("/") {
        return true;
    }else{
        return false;
    }
}

enum OperationType{
    Add,
    Substract,
    Multiply,
    Divide,
    Number
}

struct Operation {
    first: Option<Box<Operation>>,
    operation: OperationType,
    second: Option<Box<Operation>>,
    value:f32,
}

impl Operation {
    fn new(&self, first: String, operation: OperationType, second: String) -> Operation {

        // TODO: Setup parsing and replace None values in the constructor

        Operation {
            first:None,
            operation:OperationType::Number,
            second:None,
            value:0.0
        }
    }

    fn parse(&self, strexp: String) -> Operation {
        // Operation Ordering
        if strexp.contains("/"){
            let index = strexp.find("/");
            // TODO: Remove this
            return Operation{
                first:None,
                operation: OperationType::Add,
                second:None,
                value:0.0
            }
        }else if strexp.contains("*") {
            // TODO: Remove this
            return Operation{
                first:None,
                operation: OperationType::Add,
                second:None,
                value:0.0
            }
        }else if strexp.contains("-"){
            // TODO: Remove this
            return Operation{
                first:None,
                operation: OperationType::Add,
                second:None,
                value:0.0
            }
        }else if strexp.contains("+"){
            // TODO: Remove this
            return Operation{
                first:None,
                operation: OperationType::Add,
                second:None,
                value:0.0
            }
        }else{
            return Operation{
                first:None,
                operation: OperationType::Add,
                second:None,
                value:0.0
            }
        }
    }
}

// This function prepares the string for parsing by deleting all whitespaces
fn remove_spaces(strexp: String) -> String {
    return strexp.replace(" ", "").replace("\n","");
}

// This function prepares the string for parsing by changing all parentheses to "(" or ")"
fn parentheses(strexp: String) -> String {
    return strexp
        .replace("[","(")
        .replace("{","(")
        .replace("]",")")
        .replace("}",")");
}

fn main() {
    welcome();
    let mut expr = String::new();

    // Enter input answer loop
    loop {
        print!(" - > ");

        io::stdout().flush().expect("Flush failed!");

        io::stdin().read_line(&mut expr).expect("Failed to read input");

        // Check if the program should exit
        if expr == "0\n" {
            break;
        }

        expr = remove_spaces(expr);
        expr = parentheses(expr);
        // expr = to_operations(expr);
        println!("{}", expr);
        
        // Clean variable data
        expr = "".to_string();
    }
}
