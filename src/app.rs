use std::{collections::HashMap, io::{Error, Result}};
use ratatui::{self, widgets::{ScrollbarState, TableState, Widget}};

#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Main, // 
    EditingCommand,
    // 
}

// For now alias a Command with String for easier understanding
type Command = String;

#[derive(Debug, Clone, Default)]
pub struct CommandContext {
    pub command_name : String,
    pub tags: Vec<String>,
    pub command: Command,
    pub variables_to_fill: Vec<String>, // May be UNEEDED : Not yet implemented
    pub variable_prefil_values:Vec<String> // May be UNEEDED
    // Variables with prefilled values ?
}


// Not used
// just for easier data visualization and if there is further extension of the app
#[derive(Default, Debug)]
pub struct CheatSheet {
    pub name: String,
    pub sheet_tag: Vec<String>,
    pub commands: Vec<CommandContext>,    
}

impl CheatSheet {
    pub fn new(
        name: String,
        sheet_tag: Vec<String>,
        commands: Vec<CommandContext>
    ) -> Self {
        Self {
            name,
            sheet_tag,
            commands,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub search_value_input: String, // String to search within command names
    pub variable_value_input: String, // Input value to pass to editable variables
    pub commands: Vec<CommandContext>, // Exhaustive list of all commands
    pub commands_after_search: Vec<CommandContext>, // List of commands matching the search. 
    // FOR NOW : a clone of CommandContext are stored in the fields. For better performance, the search feature must be rewritten
    // Using indexes of the vector field 'commands' to avoid self-referencn structure. 
    // Interesting topic about self-ref struct : https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
    pub output_command: String, // Completed command to input, the string should be prefilled wit the selected command after a search
    pub current_screen: CurrentScreen,
    /*
    TODO :  Table state, selected row should be preserved across renders
     */
    // pub search_index: usize, // USELESS since search_table_state handle it ? Index for search result element, to highlight current search result
    pub search_table_state: TableState,
    //pub scroll_state: ScrollbarState, // UNEEDED so far
    pub editcommand_table_state: TableState,
}

impl App {
    pub fn new(commands: Vec<CommandContext>) -> App {
        // Populate the App.commands here
        App {
            search_value_input: String::new(),
            variable_value_input: String::new(),
            commands: commands.clone(),
            commands_after_search: commands.clone(),
            output_command: String::new(),
            current_screen: CurrentScreen::Main,
            search_table_state: TableState::new(),
            editcommand_table_state: TableState::new(), 
        }
    }

    // uneeded ?
    pub fn toggle_editing_variable() {
        todo!()
    }

    pub fn print_command(&self) -> Result<()> {
        println!("{}", self.output_command);
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

pub fn generate_test_data() -> Vec<CommandContext> {
    let test_data = vec![
        CommandContext { 
            command_name: "Edge case - First command".to_string(), 
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            command: "nmap -p- <IP>".to_string(),
            variables_to_fill: vec!["IP".to_string()],
            variable_prefil_values: {Vec::new()}
        },
        CommandContext { 
            command_name: "nmap - UDP Full port scan".to_string(), 
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            command: "nmap -sU <IP>".to_string(),
            variables_to_fill: vec!["IP".to_string()],
            variable_prefil_values: {
                Vec::new()
            }
        },
        CommandContext { 
            command_name: "dirbuster ".to_string(), 
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            command: "dirbuster -u http://<IP>:<PORT>".to_string(),
            variables_to_fill: vec!["IP".to_string()],
            variable_prefil_values: {
                Vec::new()
            }
        },
        CommandContext { 
            command_name: "nmap - TCP Full port scan".to_string(), 
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            command: "nmap -p- <IP>".to_string(),
            variables_to_fill: vec!["IP".to_string()],
            variable_prefil_values: {
                Vec::new()
            }
        },
        CommandContext { 
            command_name: "Edge case - Last command ".to_string(), 
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            command: "dirbuster -u http://<IP>:<PORT>".to_string(),
            variables_to_fill: vec!["IP".to_string()],
            variable_prefil_values: {
                Vec::new()
            }
        }, 
    ];

    test_data
}
