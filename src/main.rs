mod app;
mod art;
mod ui;

use app::{CheatSheet, CommandContext};

use std::collections::HashMap;
use std::{error::Error, io};

use ratatui::{
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        },
        execute,
        terminal::{
            self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
    prelude::{Backend, CrosstermBackend},
    widgets::{ScrollbarState, TableState},
    Terminal,
};

use std::path::Path;

use glob::{glob, PatternError};
use include_dir::{include_dir, Dir, File};

use crate::{
    app::{generate_test_data, App, CurrentScreen},
    ui::ui,
};

static RESSOURCES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cheats");

/// Main function
// Prep & clean the terminal. Handle unexpected app exits and returns terminal to normal state
fn main() -> Result<(), Box<dyn Error>> {
    // === App prerun terminal set-up
    enable_raw_mode();
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture); // // This is a special case, normally using stdout is fine

    /*
    We are using stderr for output
    We are utilizing the fact that the stderr is piped differently than stdout
    and rendering out project in stderr & print our completed command in stdout
     */

    // === Set-up
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // === Fetch data
    let files = read_cheatsheets();
    let cheatsheets = parse_cheatsheets(files);
    let mut commands = Vec::new();
    for cheatsheet in cheatsheets {
        for commandcontext in cheatsheet.commands {
            commands.push(commandcontext);
        }
    }
    let mut app = App::new(commands);
    let res = run_app(&mut terminal, &mut app);

    // === Post run
    // restore terminal state back to normal
    /*
    Per Ratatui's doc :
    if app exists without running the following boilerplante, the terminal will act strangely
    the user will usually have to end the terminal session & start a new one
    It is important that we handle our error in such way tha we can call this last piece
    We are printing the output or the errors after "execute!(LeaveAlternateScreen)" so that
    our prints will be rendered in the "new" scren and not lost in the alternate screen we just left
    */

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor();

    if let Ok(do_print) = res {
        if do_print {
            app.print_command()?; // Exit to stdout ?
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // Main loop that draw frames into the terminal
    // Use generic type B implementing the Backend trait, to be backend agnostic

    // === Main loop with events
    // Handles search, navigation + selection of search results
    loop {
        // DEBUG : Show key code
        /*
        if let Event::Key(key) = event::read()? {
            dbg!(key.code);
        }
        */

        // Draw a frame to the terminal, runs closure for each frame
        // Should we pass a mutable or immutable borrow of app here ?
        terminal.draw(|frame| ui(frame, app))?;

        // N.B event::read() are blocking
        if let Event::Key(key_event) = event::read()? {
            // Ignore key release events
            if key_event.kind == KeyEventKind::Release {
                continue;
            }

            // Match key events depending of the screen
            match app.current_screen {
                // For now, the app state should always be either CurrentlyEditing::Search or Variable
                CurrentScreen::Main => match key_event.code {
                    // Exit on 'Esc'
                    KeyCode::Esc => {
                        return Ok(true);
                    }
                    // Select commands from command list
                    KeyCode::Up => {
                        app.next_row();
                        // From the search result, change the command being highlighted
                    }
                    KeyCode::Down => {
                        app.previous_row();
                        // From the search result, change the command being highlighted
                    }
                    KeyCode::Enter => {
                        // The search feature clone the CommandContext

                        // Selected row index must match commands_after_search array's corresponding element index
                        let selected_row_index = app.search_table_state.selected()
                        .expect("No result row is selected by default ! Double check if there are rows. And that press arrow key to select one before pressing Enter");

                        let command_context = &app.commands_after_search[selected_row_index];
                        // TODO : The real-setup of the output command should be done in the editing section once the command has been edited.
                        app.output_command = command_context.command.clone();

                        // Switch to editing on the next frame
                        app.current_screen = CurrentScreen::EditingCommand;
                    }
                    KeyCode::Char(char_value) => {
                        app.search_value_input.push(char_value);
                        app.update_after_search();
                        app.search_table_state.select(Some(0)); // Rest the search index
                    }
                    KeyCode::Backspace => {
                        app.search_value_input.pop();
                        app.update_after_search();
                    }
                    _ => {}
                },
                CurrentScreen::EditingCommand => {
                    match key_event.code {
                        // On 'Esc', go back to main screen, flush characters in editing string
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.variable_value_input = String::new(); // flush existing string input fed during variable edition
                        }
                        // End the app run once you have selected a command
                        KeyCode::Enter => {
                            return Ok(true);
                        }
                        KeyCode::Char(char_value) => {
                            app.variable_value_input.push(char_value);
                            todo!(
                                "
                            In ui()
                            - Show variable value being inputed
                            - Dynamically completed the command"
                            );
                        }
                        KeyCode::Backspace => {
                            app.variable_value_input.pop();
                            todo!(
                                "
                            In ui()
                            - Show variable value being inputed
                            - Dynamically completed the command"
                            );
                        }
                        // Variable edition navigation
                        KeyCode::Up => {
                            todo!("Circle through the varibles to edit")
                        }
                        KeyCode::Down => {
                            todo!("Circle through the varibles to edit")
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn read_cheatsheets() -> Vec<&'static include_dir::File<'static>> {
    // Define the glob pattern to match files with the .md extension
    let glob_pattern = "**/*.md";
    let mut files: Vec<&'static include_dir::File> = vec![];

    for dirEntry in RESSOURCES_DIR.find(glob_pattern).unwrap() {
        if let Some(file) = RESSOURCES_DIR.get_file(dirEntry.path()) {
            files.push(file);
            //println!("File path {}", file.path().display());
            //println!("{}", file.contents_utf8().unwrap());
        }
    }
    files
}

pub fn parse_cheatsheets(files: Vec<&File<'static>>) -> Vec<CheatSheet> {
    // CheatSheet to populate
    let mut cheatsheets: Vec<CheatSheet> = Vec::new();

    // CommandContext fields to populate
    let mut command_name = String::new();
    let mut tags: Vec<String> = Vec::new();
    let mut command = String::new();
    // Other fields are left blanks for now

    for file in files {
        let contents = match file.contents_utf8() {
            Some(content) => content,
            None => {
                println!("No content found for {}", file.path().display());
                ""
            }
        };
        let lines: Vec<String> = contents.lines().map(String::from).collect();

        // New cheatsheet to fill
        let mut cheatsheet = CheatSheet::default();

        let tags_mapping = generate_tags_mapping().unwrap();

        // Line reading states
        let mut is_parsing_cmd: bool = false;
        let mut multiline_cmd: bool = false;

        // Populate
        let mut commands: Vec<CommandContext> = Vec::new();

        for line in lines {
            // Ugly cleaning
            let cleaned_line = line.trim().to_string();
            //dbg!("Cur line {}", &line);

            if cleaned_line.is_empty() {
            } else if cleaned_line.starts_with("# ") {
                // Cheatsheet name
                let cheatsheet_name = cleaned_line.replace("# ", "");
                cheatsheet.name = cheatsheet_name;
            } else if cleaned_line.starts_with("% ") {
                // Cheatsheet tags
                let mut cheatsheet_tags: Vec<String> = cleaned_line
                    .replace("% ", "")
                    .split(',')
                    .map(|e| e.to_string())
                    .collect();
                cheatsheet_tags = cheatsheet_tags.into_iter().map(|e| e.to_string()).collect();
                cheatsheet.sheet_tag = cheatsheet_tags;
            } else if cleaned_line.starts_with("## ") {
                command_name = cleaned_line.replace("## ", "");
            } else if cleaned_line.starts_with("#") {
                // Command tags
                let tags_beforematch: Vec<String> = cleaned_line
                    .replace('#', "")
                    .split(' ')
                    .map(|e| e.to_string())
                    .collect();
                //tags = tags.into_iter().map(|e| {e.to_string() }).collect();

                // More compact tags representation, if a matching exists
                for tag in tags_beforematch {
                    match tags_mapping.get(&tag) {
                        Some(tag) => tags.push(tag.to_string()),
                        None => tags.push(tag.to_string()),
                    }
                }
                // dbg!(&tags);
            } else if cleaned_line.contains("```") && !is_parsing_cmd {
                // Start command parsing
                is_parsing_cmd = true;
            } else if cleaned_line.contains("```") && is_parsing_cmd {
                // Stop command parsing
                is_parsing_cmd = false;
                multiline_cmd = false;
                //dbg!(&command);

                commands.push({
                    CommandContext {
                        command_name,
                        tags,
                        command,
                        variables_to_fill: Vec::new(),
                        variable_prefil_values: Vec::new(),
                    }
                });
                command_name = String::new();
                tags = Vec::new();
                command = String::new();
            } else if is_parsing_cmd {
                // command
                // TODO : Argument prefill / Dynamic argument completion

                // if the other lines are a command, is evaluated to true
                if multiline_cmd {
                    command.push_str(";\n");
                }
                command.push_str(&cleaned_line);
                multiline_cmd = true;
            }
        }
        //dbg!(&commands);
        cheatsheet.commands = commands;
        cheatsheets.push(cheatsheet);
    }
    cheatsheets
}

pub fn generate_tags_mapping() -> Result<HashMap<String, String>, serde_json::Error> {
    // JSON text
    let tags_dict = r#"
            {"target/local": "Loc",
            "target/remote": "Rem",
            "target/serve": "Ser",
            "plateform/linux": "[L] ",
            "plateform/windows": "[W] ",
            "plateform/mac": "[M] ",
            "plateform/multiple": "[*] "}
        "#;
    // Deserialize the JSON text into a HashMap
    let mapping: HashMap<String, String> = serde_json::from_str(tags_dict)?;
    Ok(mapping)
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, process::exit, result};

    use app::{App, CheatSheet, CommandContext};

    use super::*;
    #[test]
    fn test_dry_run() -> Result<(), Box<dyn Error>> {
        // === Logging

        enable_raw_mode();
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture); // // This is a special case, normally using stdout is fine

        // === Set-up
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

        // === Create app and run
        let commands = generate_test_data();
        let mut app = App::new(commands);

        let res = run_app(&mut terminal, &mut app);

        // === Post run
        // restore terminal state back to normal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;
        terminal.show_cursor();

        if let Ok(do_print) = res {
            if do_print {
                app.print_command()?; // Exit to stdout ?
            }
        } else if let Err(err) = res {
            println!("{err:?}");
        }

        // === Test only
        Ok(())
    }

    #[test]
    fn test_include_dir() {
        let files = read_cheatsheets();
        let cheatsheets = parse_cheatsheets(files);
        dbg!(&cheatsheets);
    }

    #[test]
    fn test_fuzz() {
        use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
        let matcher = SkimMatcherV2::default();

        let files = read_cheatsheets();
        let cheatsheets = parse_cheatsheets(files);

        let query = "responder";
        let mut candidates = Vec::new();

        for cheatsheet in &cheatsheets {
            for command in &cheatsheet.commands {
                candidates.push(&command.command);
            }
        }

        let mut results = candidates
            .iter()
            .filter(|&candidate| matcher.fuzzy_match(candidate, query).is_some())
            .collect::<Vec<_>>();

        results.sort_by_key(|&candidate| matcher.fuzzy_indices(candidate, query).unwrap().0);
        results.reverse();
        println!("Fuzzy search results for '{}':", query);
        for result in results {
            println!("{}", result);
        }
    }

    #[test]
    fn test_fuzz2() {
        use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
        let matcher = SkimMatcherV2::default();

        let files = read_cheatsheets();
        let cheatsheets = parse_cheatsheets(files);
        let mut commands = Vec::new();

        for cheatsheet in cheatsheets {
            for command in cheatsheet.commands {
                commands.push(command);
            }
        }

        let mut app = App::new(commands);

        let mut candidates = Vec::new();
        for commandcontext in &app.commands {
            candidates.push(commandcontext.command.clone());
        }

        let query = "responder";

        let mut results = candidates
            .iter()
            .filter(|&candidate| matcher.fuzzy_match(candidate, query).is_some())
            .collect::<Vec<_>>();

        results.sort_by_key(|&candidate| matcher.fuzzy_indices(candidate, query).unwrap().0);
        results.reverse();
        let string_results: Vec<String> = results
            .iter()
            .map(|str_slice| str_slice.to_string())
            .collect();

        /*
        println!("Fuzzy search results for '{}':", query);
        for result in results {
            println!("{}", result);
        }
         */
        let mut commands_after_search = Vec::new();

        for result in &string_results {
            for commandcontext in &app.commands {
                if commandcontext.command.as_str() == result {
                    commands_after_search.push(commandcontext.clone());
                }
            }
        }
        app.commands_after_search = commands_after_search;

        println!("Fuzzy search results for '{}':", query);
        for result in &app.commands_after_search {
            println!("{}", &result.command);
        }
    }
}
