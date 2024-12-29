use std::{collections::HashMap, fmt::Debug, io::{Error, Result}};
use ratatui::{self, widgets::{ScrollbarState, TableState, Widget}};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};


// Wrapper to implement Debug trait
pub struct SkimMatcherV2Wrapper {
    matcher: SkimMatcherV2,
}

impl Debug for SkimMatcherV2Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkimMatcherV2Wrapper")
        .field("matcher", &"SkimMatcherV2") // Placeholder for now
        .finish()
    }
}

impl SkimMatcherV2Wrapper {
    pub fn new() -> Self {
        Self { matcher: SkimMatcherV2::default() }
    }    
}

#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Main, // Main screen + search
    EditingCommand, // TODO
}

// For now alias a Command with String for easier understanding
type Command = String;

#[derive(Debug, Clone, Default)]
pub struct CommandContext {
    pub command_name : String,
    pub tags: Vec<String>,
    pub command: Command,
    pub variables_to_fill: Vec<String>, // UNEEDED ? : todo
    pub variable_prefil_values:Vec<String> // UNEEDED ?
    // field for variables with prefilled values ?
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
    // pub search_index: usize, // USELESS since search_table_state handle it ? Index for search result element, to highlight current search result
    pub search_table_state: TableState,
    pub editcommand_table_state: TableState,
    matcherwrapper: SkimMatcherV2Wrapper,
}

impl App {
    pub fn new(commands: Vec<CommandContext>) -> App {
        App {
            search_value_input: String::new(),
            variable_value_input: String::new(),
            commands: commands.clone(),
            commands_after_search: commands.clone(),
            output_command: String::new(),
            current_screen: CurrentScreen::Main,
            search_table_state: TableState::new(),
            editcommand_table_state: TableState::new(),
            matcherwrapper: SkimMatcherV2Wrapper::new(), 
        }
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
    }

    pub fn update_after_search(&mut self) {
        let matcher = &self.matcherwrapper.matcher;

        let mut candidates = Vec::new();
        for commandcontext in &self.commands {
            candidates.push(commandcontext.command.clone());
        }

        // For code visibility
        let query = self.search_value_input.as_str();

        let mut results = candidates
        .iter()
        .filter(|&candidate| matcher.fuzzy_match(candidate, query).is_some())
        .collect::<Vec<_>>();

        results.sort_by_key(|&candidate| matcher.fuzzy_indices(candidate, query).unwrap().0);
        results.reverse();
        let string_results: Vec<String>  = results
            .iter()
            .map(|str_slice| str_slice.to_string())
            .collect();

            let mut new_commands_after_search = Vec::new();
            for result in &string_results {
                for commandcontext in &self.commands {
                    if commandcontext.command.as_str() == result {
                        new_commands_after_search.push(commandcontext.clone());
                    }
                }
            }
            self.commands_after_search = new_commands_after_search;
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
