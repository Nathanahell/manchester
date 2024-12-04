use std::{fmt::Debug, io::Split};

use cli_log::debug;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::{border, line::VERTICAL},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState, Widget},
    DefaultTerminal, Frame,
};

use crate::art::BANNER;
use crate::app::{App, CurrentScreen};

// Helper function to create a centered rectangle using up a % of the available rectangle 'rect'
fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    // Cut the given rectangle into 3 vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y)/2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y)/2),
    ])
    .split(rect);
    // popup_layout[1] is the middle pice out of the 3 vertical pieces.

    // Cut the middle vertical piece into 3 width-wise piece
    // Return the middle vertical piece
    Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x)/2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x)/2),
    ]).split(popup_layout[1])[1]
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    /// UI function
    /// Used to render the differents screen parts or widgets of the application
    /// Changes in the app's internal state are reflected here !
    /// As such, ui() does not change the app structure internal state. It only "reads it".
    
    // === Main screen layout - General the Main screen chunks
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2)
        ])
        .split(frame.area());
    
    // Reassign chunks to meaningful variables for clarity
    let banner_chunk = main_chunks[0];
    let command_description_chunk = main_chunks[1];
    let prompt_chunk = main_chunks[2];
    let search_resut_chunk = main_chunks[3];
    let footer_chunk = main_chunks[4];
    
    // === Displaying a cool banner
    /*
    let banner_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    */
    let banner = Paragraph::new(BANNER)
    .alignment(Alignment::Center);

    frame.render_widget(banner,banner_chunk);

    // === Show search results
    // Display the live results in 3 columns corresponding to the command related info
    let search_resut_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    // List all search results 
    // N.B : The rows generation is placed before the other widgets so that they can reuse the results
    
    // TODO 
    // For example, rows should be filled upon app creation ?
    //let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];    
    
    let mut table_rows = vec![];
    for command_context in &app.commands_after_search {
        // todo: convert CommandContext.tags into a single string
        let row = Row::new([
            Cell::from("Dummy tags"),
            Cell::from(command_context.command_name.clone()),
            Cell::from(command_context.command.clone()), 
        ]);
        table_rows.push(row);
    }
    
    // Column widths are constrained in the same way as Layout..
    let table_widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(70),
    ];

    debug!("{table_rows:?}");
    debug!("App state : {:#?}", app);
    
    // Note: TableState is stored in my application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    let table = Table::new(table_rows, table_widths)
    // ...and they can be separated by a fixed spacing.
    .column_spacing(10)
    // You can set the style of the entire Table.
    .style(Style::new().white())
    // It has an optional header, which is simply a Row always visible at the top.
    /*
    .header(
        Row::new(vec!["Tags", "Description", "Command"])
            .style(Style::new().bold())
            // To add space between the header and the rest of the rows, specify the margin
            .bottom_margin(1),
    )
    */
    // As any other widget, a Table can be wrapped in a Block.
    //.block(Block::new().title("Table"))

    // The selected row, column, cell and its content can also be styled.
    .row_highlight_style(Style::new().reversed())
    .column_highlight_style(Style::new().red())
    .cell_highlight_style(Style::new().blue())
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>")
    .block(search_resut_block);

    frame.render_stateful_widget(table,search_resut_chunk, &mut app.search_table_state);

    // === Showing a cmd + it description
    let command_description_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    if let Some(selected_row_index) = app.search_table_state.selected() {
        let command_context = &app.commands_after_search[selected_row_index];

        let command_description_text = vec![
            Line::from(vec![
                Span::styled(&command_context.command_name, Style::new().yellow()),
            ]),
            Line::from(vec![
                Span::styled(&command_context.command, Style::new().white())
            ]),
        ];
        let command_description_widget = Paragraph::new(command_description_text)
        .block(command_description_block);
        frame.render_widget(command_description_widget,command_description_chunk);
    }

    // === Search prompt
    let prompt_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    let search_bar = Paragraph::new(app.search_value_input.clone())
    .style(Style::new().white().on_black())
    .alignment(Alignment::Left)
    .block(prompt_block);

    frame.render_widget(search_bar,prompt_chunk);


    // === Edition pop-up
    if CurrentScreen::EditingCommand == app.current_screen {
        if let Some(selected_row_index) = app.search_table_state.selected() {
            frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn

            let popup_area = centered_rect(60,50, frame.area());
            
            // Debug
            let popup_block = Block::default()
            .title("Command edition")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
            frame.render_widget(popup_block, popup_area);

            let pop_up_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(70),
                    Constraint::Percentage(20)
                ])
                .split(popup_area);

            let command_chunk = pop_up_chunks[0];   
            let command_table_chunk = pop_up_chunks[1];
            let tags_chunk = pop_up_chunks[2];

            // Clone command context to make temporary changes
            let command_context = app.commands_after_search[selected_row_index].clone();

            // Todo : Add command highlighing while editing ? Add a field in the app to keep track of CommandTable ?
            let command_paragraph = Paragraph::new(
                Span::styled(format!(">> {} ", &command_context.command), Style::default())
            );

            debug!("{:#?}", &command_context);

            // Todo : generate a table dynamically based on the selected command to allow variable edition
            //let editcommand_table = Table::new(rows, widths) ...
            //frame.render_stateful_widget(editcommand_table,search_resut_chunk, &mut app.editcommand_table_state);

            let mut tags_text = String::new();
            for tag in &command_context.tags {
                tags_text.push_str(&format!("[{}] ", tag));
            }
            let tags_paragraph = Paragraph::new(
                Span::styled(tags_text, Style::default())
            );
            
            frame.render_widget(command_paragraph, command_chunk);
            frame.render_widget(tags_paragraph, tags_chunk);
            //frame.render_widget(editcommandtable...?, command_table_chunk);
        }
    }

    // === Help bar footer
    // Debug, to delete : Render footer block to visualize space
    /*
    let footer_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());
    frame.render_widget(footer_block, footer_chunk);
    */


    // Navigation helps
    let footer_subchunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(10),
        Constraint::Percentage(90),
    ])
    .split(footer_chunk);

    let mode_subchunk = footer_subchunks[0];
    let help_subchunk = footer_subchunks[1];

    // Mode status
    let current_navigation_text = match app.current_screen 
        {
            CurrentScreen::Main => Span::styled("Search Mode", Style::default().fg(Color::Green)),
            CurrentScreen::EditingCommand => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
        }
        .to_owned(); 

    let mode_block = Paragraph::new(Line::from(current_navigation_text))
    .block(Block::default().borders(Borders::NONE));

    let help_block = Paragraph::new(
        Span::styled("(Esc) quit | (Enter) to select command | Type to search | (↑) move up | (↓) move down", Style::default().fg(Color::Yellow)
    ))
    .style(
        Style::new()
            .bg(Color::DarkGray),
    )
    .centered()
    .block(
        Block::default().borders(Borders::NONE)
    );

    frame.render_widget(mode_block, mode_subchunk);
    frame.render_widget(help_block, help_subchunk);

    // === Command editing
    // To do after
}