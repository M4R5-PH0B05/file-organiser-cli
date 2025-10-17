use std::io::{self, Write};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
// use std::path::PathBuf;
// use serde::{Serialize, Deserialize};
// use dirs;
use colored::*;

fn main() {
    const TAGS_FILE: &str = ".file_organiser/tags.json";
    let path = dirs::home_dir().unwrap().join(TAGS_FILE);

    // Define tags map
    let mut tags: HashMap<String, String>;

    // Check if the file exists
    if path.exists() {
        println!("Tag file found. Loading tags...");
        let file_contents = fs::read_to_string(&path)
            .expect("Problem reading tags file");
        // Convert to JSON
        tags = serde_json::from_str(&file_contents)
            .expect("Failed to parse tags file as JSON");
    // If no JSON file could be found
    } else {
        println!("No existing tag file found. Starting with empty tags.");
        tags = HashMap::new();
    }

    loop {
        // Main introduction
        println!("\n==== CLI FILE ORGANISER ====");
        // Take input and convert to an integer
        let choice = input_num("1) Move File\n2) Manage Tags\n3) View Tags\n4) Info\n5) Exit");
        match choice {
            1 => {
                // Enter the choice loop
                loop {
                    println!{"{}","Type EXIT to exit.".red()}
                    // Ask for a file path for the file
                    let file_to_move = input_str("Please input the path ( Relative paths allowed ) to the file that you would like to move:");
                    // Ask for a tag to apply to the file
                    if file_to_move.to_uppercase() == "EXIT" {
                        break;
                    }
                    println!("Which tag would you like to apply to this file? ");
                    // Option to SHOW all tags
                    let tag_to_apply = input_str("Type 'SHOW' for a list of your tags.");
                    // Show tags if asked for
                    if tag_to_apply.to_uppercase() == "SHOW" {
                        show_tags(&tags)
                    // If a tag is entered
                    } else if tag_to_apply.to_uppercase() == "EXIT" {
                        break;
                    } else {
                        // Check to see if the tag actually exists
                        if let Some(path) = tags.get(&tag_to_apply) {
                            // Try to move the file
                            match move_file(&file_to_move, path) {
                                Ok(_) => println!("{}","File moved successfully!".green().bold()),
                                Err(e) => println!("Failed to move file: {}", e),
                            }
                        // Exit the choice loop
                        break
                        } else {
                            println!("That tag could not be found.")
                        }


                }
                }


            }
            2 => {
                // Manage Tags
                loop {
                    println!("\n---- Manage Tags ----");
                    let choice = input_num("1) Add a new Tag\n2) Remove a Tag\n3) Go Back");
                    match choice {
                        1 => {
                            // Add a new Tag
                            // Get tag name and directory
                            let tag_name = input_str("Tag Name:");
                            let tag_directory = input_str("What FULL file path will this Tag point to? ( NOT RELATIVE ) ");
                            // ± Will need to validate the path
                            let path = Path::new(&tag_directory);
                            if !path.exists() {
                                println!("{}","That path does not exist.".red().bold());
                            } else {
                                // Insert new TAG:DIRECTORY key value pair into hashmap
                                tags.insert(tag_name,tag_directory);
                                save_tags(&tags,&TAGS_FILE);
                                // Show output
                                println!("{}","Tag added successfully!".green().bold())
                            }

                        }
                        2 => {
                            // Remove a Tag
                            println!("Name of Tag to remove: ");
                            let tag_name = input_str("Type 'SHOW' for a list of your tags");

                            // Check that the tag actually exists
                            if tag_name.to_uppercase() == "SHOW" {
                                show_tags(&tags)
                                // If a tag is entered
                            } else {
                            if let Some(_tag) = tags.get(&tag_name) {
                                // Enter removal loop
                                loop {
                                    // Confirmation message
                                    println!("{}{}{}{}{}","To Confirm: You want to remove '".bright_red().bold(),tag_name.bright_red().bold(),"'?".bright_red().bold()," ","This cannot be undone.".bright_red().bold().underline());
                                    let confirmation = input_str("( Y / N )");
                                    // Confirmation match
                                    match confirmation.to_uppercase().as_str() {
                                        // If they DO want to remove
                                        "Y" => {
                                            // Try to remove the tag
                                            match tags.remove(&tag_name) {
                                                Some(_path) => println!("{}","Tag was removed successfully.".green().bold()),
                                                None => println!("{}","Tag could not be removed. Please try again.".red().bold())
                                            }
                                            save_tags(&tags,&TAGS_FILE);
                                            // Exit the loop
                                            break;
                                        }
                                        // If they DONT want to remove
                                        "N" => {
                                            // Exit the loop
                                            break;
                                        }
                                        _ => {
                                            println!("That was not a valid option. Please Choose either Y/N.")
                                        }
                                    }
                                }
                            } else {
                                println!("{}","There was no tag found with that name.".red().bold())
                            }
                        }


                        }
                        3 => {
                            break;
                            // Go Back

                        }
                        _ => {
                            println!("\nThat is not a valid option! Please choose a number 1-3.")
                        }
                    }
                }

            }
            3 => {
                // View Tags
                show_tags(&tags)
            }
            4 => {
                // Info
                println!("\nThis tool allows you to bind 'Tags' to paths, to allow for easier file sorting. You simply bind a tag to a path to a directory, and then when sorting through files, you simply pick a file, choose a tag, and the tool will move the file to the directory that the tag corresponds to! Simple! || Made by @mars_phobos")
            }
            5 => {
                // Exit
                break
            }
            _ => {
                println!("\nThat is not a valid option! Please choose a number 1-5.")
            }
        }

    }
}



fn move_file(file_path: &str, dest_dir: &str) -> Result<(), String> {
    // Get the source of the file
    let source = Path::new(file_path);
    // Check if it sexists
    if !source.exists() {
        return Err(format!("Source file '{}' does not exist.", file_path));
    }
    // Make sure the file is valid
    let file_name = match source.file_name() {
        Some(name) => name,
        None => return Err("Invalid source file path.".to_string()),
    };
    // Create the full path
    let destination = Path::new(dest_dir).join(file_name);
    // If the directory doesn't exist, make it
    if let Some(parent) = destination.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Could not create destination directory: {}", e))?;
        }
    }
    // Move the file
    fs::rename(&source, &destination)
        .map_err(|e| format!("Failed to move file: {}", e))?;

    Ok(())
}

// Helper function to take input and return string
fn input_str(print: &str) -> String{
    // Print message
    println!("{}",print);
    // Create storage variable
    let mut input_str = String::new();
    // Read input
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    // Return input
    input_str.trim().to_string()
}

// Helper function to take input and return integer
fn input_num(prompt: &str) -> i32 {
    // Enter Loop
    loop {
        // Print message
        print!("{} ", prompt);
        io::stdout().flush().ok();
        // Create storage variable
        let mut input_str = String::new();
        // Try and read input
        if let Err(_) = io::stdin().read_line(&mut input_str) {
            println!("Couldn't read input. Please try again.");
            continue;
        }
        // Trim it
        let trimmed = input_str.trim();
        // Convert it to an integer
        match trimmed.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("`\nInvalid input: `{}`. Please enter a whole number.", trimmed),
        }
    }
}

// A helper function to list all tags
fn show_tags(tags: &HashMap<String,String>) {
    println!("\n---- Current Tags ----");
    // List Tags
    for (tag, path) in tags {
        println!("'{}' : '{}'",tag,path);
    }
}
// A helper function to save tags to the JSON file
fn save_tags(tags: &HashMap<String, String>, tags_file: &str) {
    // Build the full path to ~/.file_organiser/tags.json
    let path = dirs::home_dir().unwrap().join(tags_file);

    // Make sure the parent folder exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .expect("Failed to create tag directory");
        }
    }

    // Convert HashMap to JSON string
    let json_text = serde_json::to_string_pretty(tags)
        .expect("Failed to convert tags to JSON");

    // Save JSON to the file
    fs::write(path, json_text)
        .expect("Failed to write tags to file");
}