use std::{collections::HashMap, io};

use data::{
    chem_tree::ChemTree,
    chemicals::{BASES, BASES_MAP},
    search_engine::*,
};

use crate::print::print_dispenser_format;

pub fn start_cli(maps: &Maps, reaction_trees: &Box<HashMap<String, ChemTree>>) {
    let mut toggle = false;

    'cli: loop {
        println!("Enter your input, or type '/help' to see commands");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
        let clean = clean_input(user_input.trim().to_lowercase().to_string());

        if clean.is_empty() {
            println!("Please input a chemical to display or a command with '/'")
        } else if !clean.is_empty() && clean.chars().next().unwrap() == '/' {
            let command = &clean[1..clean.len()];
            let words: Vec<&str> = command.split_ascii_whitespace().collect();
            match words.first() {
                Some(w) => match w.to_lowercase().as_str() {
                    "q" | "quit" => break 'cli,
                    "t" | "toggle" => {
                        if toggle == true {
                            println!("Showing recipes without a %");
                            toggle = false
                        } else {
                            println!("Showing recipes as a %");
                            toggle = true
                        }
                    }
                    "h" | "help" => print_help(),
                    "b" | "bases" => println!("Available Bases: {:?}", BASES),
                    "r" | "requires" => match words.get(1) {
                        Some(w) => {
                            requires(maps, w);
                        }
                        None => println!("This command requires an argument!"),
                    },
                    _ => println!("Unknown command: {:?}", words),
                },
                None => println!("Missing command after /"),
            }
        } else {
            //check if result and reaction are same to prevent ignoring alternate recipes seperately defined
            match maps.search_map.get(&clean) {
                Some(x) => {
                    let direct = reaction_trees.get(&clean);
                    match direct {
                        Some(x) => print_dispenser_format(x.clone(), toggle),
                        None => {
                            let search = sql_search(&clean);
                            match search {
                                Ok(s) => {
                                    let fuzzy = collision_select(&s);
                                    let search_result = reaction_trees.get(&fuzzy);
                                    match search_result {
                                        Some(x) => print_dispenser_format(x.clone(), toggle),
                                        None => {}
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
                None => {
                    let direct = reaction_trees.get(&clean);
                    match direct {
                        Some(x) => print_dispenser_format(x.clone(), toggle),
                        None => {
                            let search = sql_search(&clean);
                            match search {
                                Ok(s) => {
                                    let fuzzy = collision_select(&s);
                                    let search_result = reaction_trees.get(&fuzzy);
                                    match search_result {
                                        Some(x) => print_dispenser_format(x.clone(), toggle),
                                        None => {}
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn requires(maps: &Maps, w: &str) {
    let lookup = match BASES_MAP.get(w) {
        Some(_) => w.to_string(),

        // collision_select(&sql_search(&w.to_string()).unwrap())
        None => fuzzy_search(&w.to_string(), &maps),
    };
    let uses = maps.uses_map.get(&lookup);
    match uses {
        Some(r) => {
            println!("\"{}\" is required by {:?}", lookup, r)
        }
        None => println!("\"{}\" is required by nothing.", lookup),
    }
}

fn print_help() {
    println!("\nCommands:\n---------");
    println!("/(r)equires\n\t\tDisplays all reactions required by given chem.");
    println!("/(b)ases\n\t\tDisplays all bases used in-game.");
    println!("/(h)elp\n\t\tDisplays this help page.");
    println!("/(q)uit\n\t\tQuits the program.");
    println!("---------");
}
