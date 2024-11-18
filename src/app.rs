use std::{collections::HashMap, io::{Error, Result}};

#[derive(Debug)]
pub enum CurrentScreen {
    Main, // 
    EditingCommand,
    // 
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Search,
    CommandVariable,
}

// For now alias a Command with String for easier understanding
type Command = String;

#[derive(Debug)]
pub struct CommandContext {
    pub command_name : String,
    pub tags: Vec<String>,
    pub command: Command,
    pub variables_to_fill: Vec<String>, // Not yet implemented
    pub variable_prefil_values:Option<HashMap<String, String>> // Not yet implemented
}

// Not used
// just for easier data visualization and if there is further extension of the app
pub struct CheatSheet {
    pub name: String,
    pub sheet_tag: Vec<String>,
    pub commands: Vec<CommandContext>,    
}

#[derive(Debug)]
pub struct App {
    pub search_value_input: String, // String to search within command names
    pub variable_value_input: String, // Input value to pass to editable variables
    pub commands: Vec<CommandContext>,
    pub output_command: String, // Completed command to input, the string should be prefilled wit the selected command after a search
    pub current_screen: CurrentScreen,
    pub currently_editing: CurrentlyEditing,
}

impl App {
    pub fn new() -> Self {
        App {
            search_value_input: String::new(),
            variable_value_input: String::new(),
            commands: Vec::new(),
            output_command: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: CurrentlyEditing::Search,
        }
    }

    pub fn toggle_editing() {
        todo!()
    }

    pub fn print_command(&self) -> Result<()> {
        Ok(())
    }
}