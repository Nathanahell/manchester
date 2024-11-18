use std::io::Split;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::{border, line::VERTICAL},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

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

pub fn ui(frame: &mut Frame, app: &App) {
    /// UI function
    /// Used to render the differents screen parts or widgets of the application
    /// Changes in the app's internal state are reflected here !
    /// As such, ui() does not change the app structure internal state. It only "reads it".
    
    // === Main screen layout - General the Main screen chunks
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3)
        ])
        .split(frame.area());
    
    // Reassign chunks to meaningful variables for clarity
    let banner_chunk = main_chunks[0];
    let command_description_chunk = main_chunks[1];
    let prompt_chunk = main_chunks[2];
    let search_resut_chunk = main_chunks[3];
    let help_bar_chunk = main_chunks[4];
    
    // === Displaying a cool banner
    let banner_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    frame.render_widget(banner_block,banner_chunk);

    // === Showing a cmd + it description
    let command_description_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    frame.render_widget(command_description_block,command_description_chunk);

    // === Search prompt
    let prompt_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    frame.render_widget(prompt_block,prompt_chunk);

    // === Show search results
    // Display the live results in 3 columns corresponding to the command related info
    let search_resut_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    frame.render_widget(search_resut_block,search_resut_chunk);

    // List all search results

    // === Help bar
    let help_bar_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    frame.render_widget(help_bar_block,help_bar_chunk);


    // === Command Pop-up
    // To do after 

}