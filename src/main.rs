use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        help();
    } else if args[1] == "-h" {
        help();
    } 

    parsing(args);
}


fn parsing(args: Vec<String>) {
    let stack = args[1].trim().parse::<u32>().unwrap_or_else(|_error| {
        println!("Invalid stack size!");
        exit(1);
    });

    let items = args[2].trim().parse::<u32>().unwrap_or_else(|_error| {
        println!("Invalid amount of items!");
        exit(1);
    });

    if stack != 64 && stack != 16 {
        println!("{stack:?} is not a valid stack size.\nTry 64 or 16.");
        exit(1);
    } 


    calc(stack, items);
}


fn calc(stack: u32, items: u32) {
    let stacks = items / stack;
    let remainder = items % stack;

    println!("\nStacker 1.0.0\n");
    println!("{items:?} items = {stacks:?} stacks and {remainder:?} items\n({stacks:?} x {stack:?} + {remainder:?} items)");

    exit(0);
}

fn help() {
    println!("\nStacker 1.0.0\n");
    println!("Usage:\nstacker <stack size> <items>\n");
    println!("Open help:\nstacker -h");
    println!("\n-----------------------------\n");
    println!("Stacker made by ubionexd\nStacker 1.0.0 is licensed under: The Unlicense\nSource code: https://github.com/ubionexd/stacker\n");
    exit(0);
}