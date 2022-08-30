use std::{env, process::exit};
use colored::*;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Catch not enough args
    if args.len() != 4 {
        help();
    }

    // Select operation
    if args[1] == "stacks" {
        stacks(args);
    } else if args[1] == "items" {
        items(args);
    } else {
        // Catch incorrect operation
        println!("{} Incorrect operation specified.\nRun {} without any arguments to get help!", "Error:".red(), "stacker".on_color("gray"));
        exit(1);
    }

}

fn help() {
    println!("Stacker 2.0");
    println!("\n\nUsage:\nstacker <method> <stack size> <input>");
    println!("\nMethods:\n- stacks: Converts the amounts of items to stacks, the input value is the amount of items.\n- items: Converts the amount of stacks to items, the input value is the amount of stacks.");
    println!("\nStack sizes:\n- 64: Chests, Dirt, etc.\n- 16: Snowballs, Buckets, etc.");
    exit(0);
}

// Stacks operation, expects a vector of strings as arguments
fn stacks(args: Vec<String>) {
    // Parse arguments
    let stack_size = args[2].trim().parse::<u32>().unwrap_or_else(|_error| {
        println!("{} Invalid stack size!", "Error:".red());
        exit(1);
    });

    let items = args[3].trim().parse::<u32>().unwrap_or_else(|_error| {
        println!("{} Invalid amount of items!", "Error:".red());
        exit(1);
    });

    // Catch invalid stack sizes
    if stack_size != 64 && stack_size != 16 {
        println!("{} {stack_size:?} is not a valid stack size.\nRun {} without any arguments to get help!", "Error:".red(), "stacker".on_color("gray"));
        exit(1);
    }

    // Calculate
    let stacks = items / stack_size;
    let remainder = items % stack_size;

    println!("{items:?} items = {stacks:?} stacks and {remainder:?} items\n({stacks:?} x {stack_size:?} + {remainder:?} items)");
    exit(0);
}

// Items operation, expects a vector of strings as arguments
fn items(args: Vec<String>) {
    // Parse arguments
    let stack_size = args[2].trim().parse::<u32>().unwrap_or_else(|_error| {
        println!("{} Invalid stack size!", "Error:".red());
        exit(1);
    });

    let stacks = args[3].trim().parse::<u32>().unwrap_or_else(|_error| {
        println!("{} Invalid amount of sctacks!", "Error:".red());
        exit(1);
    });

    // Catch invalid stack sizes
    if stack_size != 64 && stack_size != 16 {
        println!("{} {stack_size:?} is not a valid stack size.\nRun {} without any arguments to get help!", "Error:".red(), "stacker".on_color("gray"));
        exit(1);
    }

    // Calculate
    let items = stacks * stack_size;

    println!("{stacks:?} stacks (of {stack_size:?} items) = {items:?} items");
    exit(0);
}