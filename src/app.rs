use std::{collections::HashMap, io::{Error, Result}};
use ratatui::{self, widgets::{ScrollbarState, TableState, Widget}};

#[derive(Debug)]
pub enum CurrentScreen {
    Main, // 
    EditingCommand,
    // 
}

// For now alias a Command with String for easier understanding
type Command = String;

#[derive(Debug, Clone)]
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
    pub commands: Vec<CommandContext>, // Exhaustive list of all commands
    pub commands_after_search: Vec<CommandContext>, // List of commands matching the search
    pub output_command: String, // Completed command to input, the string should be prefilled wit the selected command after a search
    pub current_screen: CurrentScreen,
    /*
    TODO :  Table state, selected row should be preserved across renders
     */
    // pub search_index: usize, // USELESS : Index for search result element, to highlight current search result
    pub search_table_state: TableState,
    pub scroll_state: ScrollbarState, 
}

impl App {
    pub fn new() -> Self {
        // Populate the App.commands here
        todo!();

        /*
        App {
            search_value_input: String::new(),
            variable_value_input: String::new(),
            commands: Vec::new(),
            commands_after_search: Vec::new(),
            output_command: String::new(),
            current_screen: CurrentScreen::Main,
            search_table_state: TableState::new(),
            scroll_state: ScrollbarState::new(commands_after_search.len() - 1)
        }

        */
    }

    // uneeded ?
    pub fn toggle_editing_variable() {
        todo!()
    }

    pub fn print_command(&self) -> Result<()> {
        Ok(())
    }

    pub fn previous_row(&mut self) {
        let index = match self.search_table_state.selected() {
            Some(previous_selected_index) => {
                if previous_selected_index >= self.commands_after_search.len() - 1 {
                    0
                } else {
                    previous_selected_index + 1
                }
            }
            None => 0,
        };
        self.search_table_state.select(Some(index));
        //self.scroll_state = self.scroll_state.position(index * ITEM_HEIGHT);
    }

    pub fn next_row(&mut self) {
        let index = match self.search_table_state.selected() {
            Some(previous_selected_index)  => {
                if previous_selected_index == 0 {
                    self.commands_after_search.len() -1
                } else {
                    previous_selected_index - 1
                }
            },
            None => 0,
        };
        self.search_table_state.select(Some(index));
        //self.scroll_state = self.scroll_state.position(index * ITEM_HEIGHT);
    }
}