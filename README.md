# GWEP Chemical Search Engine
This local branch of https://github.com/megumew/gwep_chem_finder has been cleaned up for display purposes.

A chemical recipe search engine for the SpaceStation 13 "goonstation" servers

## Usage
The program retrieves the open-source configurations for chemical recipies from https://github.com/goonstation/goonstation, parses it into a Vec of Rust Structs, and then serializes this into a local SQL database for use in the code. 

This Vec of structs is then converted into a tree that contains all of the required reagents. 

The program can be run in CLI mode with -c or --cli as a launch argument ex. `cargo run -- -c`.
The program takes a reaction to display and also commands as input with "/".

Chemicals can be search by typing their name into the cli, with relevant results and their recipies appearing once entered.

Use `/help` or `/h` to see all commands.

## Goal of the program
This program aims to eventually provide a GUI experience to make creating perfect beakers simple by creating optimized chemical recipes.
