use std::io::Split;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::{border, line::VERTICAL},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Widget},
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
            Constraint::Length(5),
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

    // List all search results
    // TODO 
    // For example, rows should be filled upon app creation ?
    //let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];    
    
    let mut rows = vec![];
    for command_context in &app.commands_after_search {
        // todo: convert CommandContext.tags into a single string
        let row = Row::new([
            Cell::from("Dummy tags"),
            Cell::from(command_context.command_name.clone()),
            Cell::from(command_context.command.clone()), 
        ]);
        rows.push(row);
    }
    
    // Column widths are constrained in the same way as Layout..
    let widths = [
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(20),
    ];

    println!("{rows:?}");

    // Note: TableState is stored in my application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    let table = Table::new(rows, widths)
    // ...and they can be separated by a fixed spacing.
    .column_spacing(30)
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


    // === Help bar footer
    let footer_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    // Mode status
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Search Mode", Style::default().fg(Color::Green)),
            CurrentScreen::EditingCommand => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().white()),
        {
            match app.current_screen {
                CurrentScreen::Main => Span::styled("Searching for a command", Style::default().fg(Color::Green)),
                CurrentScreen::EditingCommand => Span::styled("Editing command", Style::default().fg(Color::Green))
            }                
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
    .block(Block::default().borders(Borders::ALL));

    // Navigation tips : show keys & actions
    let footer_subchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(footer_chunk);


    let mode_subchunk = footer_subchunks[0];
    let help_subchunk = footer_subchunks[1];

    frame.render_widget(footer_block, footer_chunk);


    // === Command editing
    // To do after 

}