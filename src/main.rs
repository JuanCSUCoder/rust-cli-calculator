use io::Write;
use std::io;

// This prints a welcome message
fn welcome() {
    println!("Welcome to this CLI Rust Calculator");
    println!("Write down the expression to solve or type 0 to exit");
}

fn has_operations(strexp: String) -> bool {
    if strexp.contains("+") || strexp.contains("-") || strexp.contains("*") || strexp.contains("/")
    {
        return true;
    } else {
        return false;
    }
}

fn to_operation(charvar: char) -> OperationType {
    match charvar {
        '+' => return OperationType::Add,
        '-' => return OperationType::Substract,
        '*' => return OperationType::Multiply,
        '/' => return OperationType::Divide,
        _ => {
            println!("Unknown operation. Write better code xd");
            return OperationType::Number;
        }
    }
}

fn is_superior_than_first(type1: OperationType, type2: OperationType) -> bool {
    let type1_number = match type1 {
        OperationType::Number => 0,
        OperationType::Add => 4,
        OperationType::Substract => 3,
        OperationType::Multiply => 2,
        OperationType::Divide => 1,
    };

    let type2_number = match type2 {
        OperationType::Number => 0,
        OperationType::Add => 4,
        OperationType::Substract => 3,
        OperationType::Multiply => 2,
        OperationType::Divide => 1,
    };

    if type2_number > type1_number {
        return true;
    } else {
        return false;
    }
}

enum OperationType {
    Add,
    Substract,
    Multiply,
    Divide,
    Number,
}

impl Clone for OperationType {
    fn clone(&self) -> OperationType {
        match self {
            OperationType::Add => OperationType::Add,
            OperationType::Substract => OperationType::Substract,
            OperationType::Multiply => OperationType::Multiply,
            OperationType::Divide => OperationType::Divide,
            OperationType::Number => OperationType::Number,
        }
    }
}

struct Operation {
    first: Option<Box<Operation>>,
    operation: OperationType,
    second: Option<Box<Operation>>,
    value: f32,
}

impl Operation {
    fn resolve(&self) -> f32{
        match self.operation {
            OperationType::Number => return self.value,
            OperationType::Add => return self.first.as_ref().unwrap().resolve() + self.second.as_ref().unwrap().resolve(),
            OperationType::Substract => return self.first.as_ref().unwrap().resolve() - self.second.as_ref().unwrap().resolve(),
            OperationType::Multiply => return self.first.as_ref().unwrap().resolve() * self.second.as_ref().unwrap().resolve(),
            OperationType::Divide => return self.first.as_ref().unwrap().resolve() / self.second.as_ref().unwrap().resolve(),
        }
    }
}

fn parse(strexp: String) -> Operation {
    let borrowstr = strexp.clone();
    if has_operations(strexp) {
        let mut superior_operation_index: usize = 0;
        let mut superior_operation_type = OperationType::Number;
        let charlist: Vec<char> = borrowstr.clone().chars().collect();
        for index in 0..borrowstr.len() {
            if has_operations(charlist[index].to_string())
                && is_superior_than_first(
                    superior_operation_type.clone(),
                    to_operation(charlist[index]),
                )
            {
                superior_operation_index = index;
                superior_operation_type = to_operation(charlist[index]);
            }
        }

        let mut before_operation = "".to_string();
        for index in 0..superior_operation_index {
            before_operation.push(charlist[index]);
        }
        let mut after_operation = "".to_string();
        for index in superior_operation_index + 1..borrowstr.len() {
            after_operation.push(charlist[index]);
        }

        let first_operation = parse(before_operation);
        let second_operation = parse(after_operation);

        return Operation {
            first: Some(Box::new(first_operation)),
            operation: superior_operation_type,
            second: Some(Box::new(second_operation)),
            value: 0.0,
        };
    } else {
        return Operation {
            first: None,
            operation: OperationType::Number,
            second: None,
            value: borrowstr.parse::<f32>().unwrap(),
        };
    }
}

// This function prepares the string for parsing by deleting all whitespaces
fn remove_spaces(strexp: String) -> String {
    return strexp.replace(" ", "").replace("\n", "");
}

// This function prepares the string for parsing by changing all parentheses to "(" or ")"
fn parentheses(strexp: String) -> String {
    return strexp
        .replace("[", "(")
        .replace("{", "(")
        .replace("]", ")")
        .replace("}", ")");
}

fn main() {
    welcome();
    let mut expr = String::new();

    // Enter input answer loop
    loop {
        print!(" - > ");

        io::stdout().flush().expect("Flush failed!");

        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read input");

        // Check if the program should exit
        if expr == "0\n" {
            break;
        }

        expr = remove_spaces(expr);
        expr = parentheses(expr);
        let operation = parse(expr);
        println!("{}", operation.resolve());
        // Clean variable data
        expr = "".to_string();
    }
}
