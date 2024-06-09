use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "file-manager", version = "1.0",
author = "Your Name <your.email@example.com>",
about = "A simple file management system with clap attributes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new file wow!
    Create {
        /// Name of the file to create
        name: String,

        /// Content to write to the file
        #[arg(short, long)]
        content: Option<String>,
    },
    /// List all files
    List {
        /// Display detailed information
        #[arg(short, long)]
        verbose: bool,
    },
    /// Delete a file
    Delete {
        /// Name of the file to delete
        name: String,
    },
}

pub fn run_example() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name, content } => {
            create_file(name, content);
        }
        Commands::List { verbose } => {
            list_files(*verbose);
        }
        Commands::Delete { name } => {
            delete_file(name);
        }
    }
}

fn create_file(name: &str, content: &Option<String>) {
    let content = content.as_deref().unwrap_or("");

    // Simulate file creation
    println!("Creating file: {}", name);
    println!("Content: {}", content);
}

fn list_files(verbose: bool) {
    // Simulate listing files
    if verbose {
        println!("Listing files with details...");
    } else {
        println!("Listing files...");
    }
}

fn delete_file(name: &str) {
    // Simulate file deletion
    println!("Deleting file: {}", name);
}
