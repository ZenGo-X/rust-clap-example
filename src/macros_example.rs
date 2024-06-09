use clap::{arg, command};


pub fn run_example() {
    let matches = command!("file-manager")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple file management system with clap macros")
        .subcommand(
            command!("create")
                .about("Create a new file")
                .arg(arg!(<NAME> "Name of the file to create"))
                .arg(arg!(-c --content <CONTENT> "Content to write to the file").required(false)),
        )
        .subcommand(
            command!("list")
                .about("List all files")
                .arg(arg!(-v --verbose "Display detailed information")),
        )
        .subcommand(
            command!("delete")
                .about("Delete a file")
                .arg(arg!(<NAME> "Name of the file to delete")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_m)) => create_file(sub_m),
        Some(("list", sub_m)) => list_files(sub_m),
        Some(("delete", sub_m)) => delete_file(sub_m),
        _ => eprintln!("Unknown command"),
    }
}


pub fn create_file(matches: &clap::ArgMatches) {
    let file_name = matches.get_one::<String>("NAME").unwrap();
    let content = matches.get_one::<String>("content").map_or("", String::as_str);

    // Simulate file creation
    println!("Creating file: {}", file_name);
    println!("Content: {}", content);
}

pub fn list_files(matches: &clap::ArgMatches) {
    let verbose = matches.get_flag("verbose");

    // Simulate listing files
    if verbose {
        println!("Listing files with details...");
    } else {
        println!("Listing files...");
    }
}

pub fn delete_file(matches: &clap::ArgMatches) {
    let file_name = matches.get_one::<String>("NAME").unwrap();

    // Simulate file deletion
    println!("Deleting file: {}", file_name);
}
