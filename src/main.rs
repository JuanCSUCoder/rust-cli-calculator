use std::io;
use io::Write;

fn welcome(){
    println!("Welcome to this CLI Rust Calculator");
    println!("Write down the expression to solve or type 0 to exit");
}

fn has_operations(strexp: String) -> bool {
    if strexp.contains("+") || strexp.contains("-") || strexp.contains("*") || strexp.contains("/") || strexp.contains("(") || strexp.contains(")"){
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
}

struct Operation {
    first: String,
    operation: OperationType,
    second: String,
}

impl Operation {
    fn new(&self, first: String, operation: OperationType, second: String) -> Operation {
        Operation {
            first:first,
            operation:operation,
            second:second,
        }
    }
    fn get_result(&self) -> i64{
        // Get numbers
        let mut first_num: i64 = 0;
        let mut second_num: i64 = 0;

        let first_str = self.first.clone();
        let second_str = self.second.clone();

        if has_operations(first_str){
            let oper1 = Operation{
                first:"".to_string(),
                operation: OperationType::Add,
                second:"".to_string(),
            };
            Operation::parse(&oper1, self.first.clone());
            first_num = Operation::get_result(&oper1);
        }else{
            first_num = self.first.parse::<i64>().unwrap();
        }

        if has_operations(second_str){
            let oper2 = Operation{
                first:"".to_string(),
                operation: OperationType::Add,
                second:"".to_string(),
            };
            Operation::parse(&oper2, self.second.clone());
            second_num = Operation::get_result(&oper2);
        }else{
            second_num = self.second.parse::<i64>().unwrap();
        }

        // Operate
        match self.operation {
            OperationType::Add => first_num + second_num,
            OperationType::Substract => first_num - second_num,
            OperationType::Multiply => first_num * second_num,
            OperationType::Divide => first_num / second_num
        }
    }

    fn parse(&self, strexp: String) -> Operation {
        // Operation Ordering
        if strexp.contains("/"){
            let index = strexp.find("/");
            // TODO: Remove this
            return Operation{
                first:strexp,
                operation: OperationType::Add,
                second:"0".to_string()
            }
        }else if strexp.contains("*") {
            // TODO: Remove this
            return Operation{
                first:strexp,
                operation: OperationType::Add,
                second:"0".to_string()
            }
        }else if strexp.contains("-"){
            // TODO: Remove this
            return Operation{
                first:strexp,
                operation: OperationType::Add,
                second:"0".to_string()
            }
        }else if strexp.contains("+"){
            // TODO: Remove this
            return Operation{
                first:strexp,
                operation: OperationType::Add,
                second:"0".to_string()
            }
        }else{
            return Operation{
                first:strexp,
                operation: OperationType::Add,
                second:"0".to_string()
            }
        }
    }
}

fn remove_spaces(strexp: String) -> String {
    return strexp.replace(" ", "").replace("\n","");
}

fn parentheses(strexp: String) -> String {
    return strexp
        .replace("[","(")
        .replace("{","(")
        .replace("]",")")
        .replace("}",")");
}

fn to_operations(expr: String) -> String {
    return expr;
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
        expr = to_operations(expr);
        println!("{}", expr);
        
        // Clean variable data
        expr = "".to_string();
    }
}
