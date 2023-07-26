use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};
include!("rust.rs");
include!("csj.rs");
include!("py.rs");
include!("node.rs");

fn main() {
    println!(
        "
    
    Choose a boilerplate:

        • csj [client side javascipt]
        • node 
        • rust 
        • py 

        "
    );

    let mut input = String::new();

    loop {
        print!("> ");

        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let boilerplate = input.trim().to_lowercase();

        match boilerplate.as_str() {
            "csj" => {
                csj();
                break;
            }
            "node" => {
                node();
                break;
            }
            "rust" => {
                rust();
                break;
            }
            "py" => {
                py();
                break;
            }
            _ => {
                println!("Please choose a valid boilerplate!");
                input.clear();
            }
        }
    }
}
