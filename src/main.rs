use std::io;
use io::Write;

fn welcome(){
    println!("Welcome to this CLI Rust Calculator");
    println!("Write down the expression to solve or type 0 to exit");
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

fn define_oper_order(expr: String) -> String {
    // Order operations by operation order without including what is inside parethesis
        // Remove Parentheses and change by letters
        // Create an array of characters from the string
        // Get the index of every operator
        // Order every index according to the operation order
        // Divide the indexes in groups according to the operation related
        // Surround with parentheses the first index
        // Repeat all the steps
    // Create parentheses around first type operation
    // Order operations by operation order without including what is inside parethesis
    // Create parentheses around second type operation
    // Repeat until ending
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
        expr = define_oper_order(expr);
        println!("{}", expr);
        
        // Clean variable data
        expr = "".to_string();
    }
}
