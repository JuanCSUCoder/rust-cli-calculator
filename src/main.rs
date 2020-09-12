use std::io;
use io::Write;

fn welcome(){
    println!("Welcome to this CLI Rust Calculator");
    println!("Write down the expression to solve or type 0 to exit");
}

fn remove_spaces(strexp: String) -> String {
    return strexp.replace(" ", "").replace("\n","");
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
        println!("{}", expr);
        
        // Clean variable data
        expr = "".to_string();
    }
}
