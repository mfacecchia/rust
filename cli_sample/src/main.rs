use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version)]
struct CliArgs {
    /// First arg
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add operation
    Add {
        /// The first number to add
        first: i32,
        /// The second number to add
        second: i32,
        /// Optional additional operation
        #[command(subcommand)]
        and: Option<OperationCommands>,
    },
}

#[derive(Debug, Subcommand)]
enum OperationCommands {
    /// Add operation
    Add { first: i32, second: i32 },
    /// Subtract operation
    Sub { first: i32, second: i32 },
}

fn main() {
    println!("Hello, world!");
    let args = CliArgs::parse();
    match args.command {
        Commands::Add { first, second, and } => {
            if first == 15 && second == 18 {
                println!("15 e 18 quanto fa?");
                return;
            }
            let sum_result = handle_add(first, second);
            match and {
                Some(and_args) => match and_args {
                    OperationCommands::Add {
                        first: first_in,
                        second: second_in,
                    } => {
                        println!("{}", handle_add(first_in, second_in) + sum_result);
                    }
                    OperationCommands::Sub {
                        first: first_in,
                        second: second_in,
                    } => {
                        println!("{}", sum_result + handle_sub(first_in, second_in));
                    }
                },
                None => {
                    println!("{}", sum_result);
                }
            }
            println!()
        }
    }
}

fn handle_add(first: i32, second: i32) -> i32 {
    first + second
}

fn handle_sub(first: i32, second: i32) -> i32 {
    first - second
}
