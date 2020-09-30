use io::Write;
use std::io;

// This prints a welcome message
fn welcome() {
    println!("Welcome to this CLI Rust Calculator");
    println!("Write down the expression to solve or type 0 to exit");
}

fn has_operations(strexp: String) -> bool {
    if strexp.contains("+")
        || strexp.contains("-")
        || strexp.contains("*")
        || strexp.contains("/")
        || strexp.contains("^")
    {
        return true;
    } else {
        return false;
    }
}

fn has_vanishable_parenthesis(str_to_check: String) -> bool {
    let mut first_open_par_index: usize = 0;
    let mut last_close_par_index: usize = 0;
    let charlist: Vec<char> = str_to_check.clone().chars().collect();
    let mut got_open_par = false;
    let mut got_close_par = false;
    let mut level: usize = 0;

    for index in 0..str_to_check.len() {
        // println!("Iter Start Value: {} Level: {} GF: {} GS: {}", charlist[index], level, got_open_par, got_close_par);
        if !got_open_par && charlist[index] == '(' {
            first_open_par_index = index;
            got_open_par = true;
        } else if charlist[index] == ')' && level == 0 && !got_close_par {
            last_close_par_index = index;
            got_close_par = true;
        } else if charlist[index] == '(' {
            level += 1;
        } else if charlist[index] == ')' {
            level -= 1;
        }
    }

    if first_open_par_index == 0
        && last_close_par_index == str_to_check.len() - 1
        && str_to_check.len() != 1
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
        '^' => return OperationType::Power,
        _ => {
            println!("Unknown operation. Write better code xd");
            return OperationType::Number;
        }
    }
}

fn is_superior_than_first(type1: OperationType, type2: OperationType) -> bool {
    let type1_number = match type1 {
        OperationType::Number => -1,
        OperationType::Add => 4,
        OperationType::Substract => 3,
        OperationType::Multiply => 2,
        OperationType::Divide => 1,
        OperationType::Power => 0,
    };

    let type2_number = match type2 {
        OperationType::Number => -1,
        OperationType::Add => 4,
        OperationType::Substract => 3,
        OperationType::Multiply => 2,
        OperationType::Divide => 1,
        OperationType::Power => 0,
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
    Power,
    Number,
}

impl Clone for OperationType {
    fn clone(&self) -> OperationType {
        match self {
            OperationType::Add => OperationType::Add,
            OperationType::Substract => OperationType::Substract,
            OperationType::Multiply => OperationType::Multiply,
            OperationType::Divide => OperationType::Divide,
            OperationType::Power => OperationType::Power,
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
    fn resolve(&self) -> f32 {
        match self.operation {
            OperationType::Number => return self.value,
            OperationType::Add => {
                return self.first.as_ref().unwrap().resolve()
                    + self.second.as_ref().unwrap().resolve()
            }
            OperationType::Substract => {
                return self.first.as_ref().unwrap().resolve()
                    - self.second.as_ref().unwrap().resolve()
            }
            OperationType::Multiply => {
                return self.first.as_ref().unwrap().resolve()
                    * self.second.as_ref().unwrap().resolve()
            }
            OperationType::Divide => {
                return self.first.as_ref().unwrap().resolve()
                    / self.second.as_ref().unwrap().resolve()
            }
            OperationType::Power => {
                return self
                    .first
                    .as_ref()
                    .unwrap()
                    .resolve()
                    .powf(self.second.as_ref().unwrap().resolve());
            }
        }
    }
}

fn parse(strexp: String) -> Operation {
    let borrowstr = strexp.clone();
    let borrowstr2 = strexp.clone();
    if !has_vanishable_parenthesis(strexp) {
        if has_operations(borrowstr2) {
            let mut superior_operation_index: usize = 0;
            let mut superior_operation_type = OperationType::Number;
            let mut level: usize = 0;
            let charlist: Vec<char> = borrowstr.clone().chars().collect();
            for index in 0..borrowstr.len() {
                if has_operations(charlist[index].to_string())
                    && is_superior_than_first(
                        superior_operation_type.clone(),
                        to_operation(charlist[index]),
                    ) && level == 0
                {
                    superior_operation_index = index;
                    superior_operation_type = to_operation(charlist[index]);
                } else if charlist[index] == '(' {
                    level += 1;
                } else if charlist[index] == ')' {
                    level -= 1;
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
    } else {
        let char_list: Vec<char> = borrowstr2.clone().chars().collect();
        let mut new_str = "".to_string();
        for index in 1..char_list.len() - 1 {
            new_str.push(char_list[index]);
        }
        return parse(new_str);
    }
}

// This function prepares the string for parsing by deleting all whitespaces
fn remove_spaces(strexp: String) -> String {
    return strexp.replace(" ", "").replace("\n", "");
}

// This function prepares the string for parsing by changing all parenthesis to "(" or ")"
fn parenthesis(strexp: String) -> String {
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
        expr = parenthesis(expr);
        let mut finalstr;
        let char_list: Vec<char> = expr.clone().chars().collect();
        if has_operations(char_list[0].to_string()) {
            finalstr = "0".to_string();
            finalstr.push_str(&expr[..]);
        } else {
            finalstr = expr;
        }
        let operation = parse(finalstr);
        println!("{}", operation.resolve());
        // Clean variable data
        expr = "".to_string();
    }
}
