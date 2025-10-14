use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::art::BANNER;

// Helper function to create a centered rectangle using up a % of the available rectangle 'rect'
fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    // Cut the given rectangle into 3 vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(rect);
    // popup_layout[1] is the middle pice out of the 3 vertical pieces.

    // Cut the middle vertical piece into 3 width-wise piece
    // Return the middle vertical piece
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// UI function
/// Used to render the differents screen parts or widgets of the application
/// Changes in the app's internal state are reflected here !
/// As such, ui() does not change the app structure internal state. It only "reads it".
pub fn ui(frame: &mut Frame, app: &mut App) {
    // === Main screen layout - General the Main screen chunks
    let main_rectangles = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(frame.area());

    // Reassign chunks to meaningful variables for clarity
    let banner_rectangle = main_rectangles[0];
    let command_description_rectangle = main_rectangles[1];
    let prompt_rectangle = main_rectangles[2];
    let search_result_rectangle = main_rectangles[3];
    let footer_rectangle = main_rectangles[4];

    // === Displaying a cool banner
    /*
    let banner_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    */
    let banner = Paragraph::new(BANNER).alignment(Alignment::Center);

    frame.render_widget(banner, banner_rectangle);

    // === Show search results
    // Display the live results in 3 columns corresponding to the command related info
    let search_resut_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // List all search results, populate table
    // N.B : The rows generation is placed before the other widgets so that they can reuse the results

    let mut table_rows = vec![];
    for command_context in &app.commands_after_search {
        let tag_string = command_context.tags.clone().join(",");
        let row = Row::new([
            Cell::from(tag_string),
            Cell::from(command_context.command_name.clone()),
            Cell::from(command_context.command.clone()),
        ]);
        table_rows.push(row);
    }

    // Column widths are constrained in the same way as Layout..
    let table_widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(50),
    ];

    // dbg!("{table_rows:?}");
    // dbg!("App state : {:#?}", &app);

    // Note: TableState is stored in my application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    let table = Table::new(table_rows, table_widths)
        // ...and they can be separated by a fixed spacing.
        .column_spacing(3)
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

    frame.render_stateful_widget(table, search_result_rectangle, &mut app.search_table_state);

    // === Showing a cmd + it description
    let command_description_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    if let Some(selected_row_index) = app.search_table_state.selected() {
        let command_context = &app.commands_after_search[selected_row_index];

        let command_description_text = vec![
            Line::from(vec![Span::styled(
                &command_context.command_name,
                Style::new().yellow(),
            )]),
            Line::from(vec![Span::styled(
                &command_context.command,
                Style::new().white(),
            )]),
        ];
        let command_description_widget =
            Paragraph::new(command_description_text).block(command_description_block);
        frame.render_widget(command_description_widget, command_description_rectangle);
    }

    // === Search prompt
    let prompt_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let search_bar = Paragraph::new(app.search_value_input.clone())
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left)
        .block(prompt_block);

    frame.render_widget(search_bar, prompt_rectangle);

    // === Edition pop-up
    if CurrentScreen::EditingCommand == app.current_screen {
        if let Some(selected_row_index) = app.search_table_state.selected() {
            frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn

            let popup_area = centered_rect(90, 60, frame.area());

            // Debug
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            frame.render_widget(popup_block, popup_area);

            let popup_edition_area = centered_rect(85, 55, frame.area());

            let pop_up_rectangles = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(5),
                    Constraint::Percentage(20),
                    Constraint::Percentage(70),
                    Constraint::Percentage(5),
                ])
                .split(popup_edition_area);

            let title_rectangle = pop_up_rectangles[0];
            let command_rectangle = pop_up_rectangles[1];
            // let command_table_rectangle = pop_up_rectangles[2];
            let tags_rectangle = pop_up_rectangles[3];

            // Clone command context to make temporary changes
            let command_context = app.commands_after_search[selected_row_index].clone();

            // TODO : Add command highlighing while editing ? Add a field in the app to keep track of CommandTable ?
            let command_paragraph =
                Paragraph::new(Line::from(format!(">> {} ", &command_context.command)))
                    .style(Style::default())
                    .wrap(Wrap { trim: true });

            // debug!("{:#?}", &command_context);

            // Command editing

            // TODO : generate a table dynamically based on the selected command to allow variable edition
            //let editcommand_table = Table::new(rows, widths) ...
            //frame.render_stateful_widget(editcommand_table,search_result_rectangle, &mut app.editcommand_table_state);

            let title_paragraph = Paragraph::new(Line::from("Command edition"))
                .style(Style::default())
                .centered();

            let mut tags_text = String::new();
            for tag in &command_context.tags {
                tags_text.push_str(&format!("[{}] ", tag));
            }
            let tags_paragraph = Paragraph::new(Span::styled(tags_text, Style::default()));

            frame.render_widget(title_paragraph, title_rectangle);
            frame.render_widget(command_paragraph, command_rectangle);
            frame.render_widget(tags_paragraph, tags_rectangle);
            //frame.render_widget(editcommandtable...?, command_table_rectangle);
        }
    }

    // === Help bar footer
    // DEBUG : Render footer block to visualize space
    /*
        let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
        frame.render_widget(footer_block, footer_rectangle);
    */

    // Navigation helps
    let footer_subchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(footer_rectangle);

    let mode_subchunk = footer_subchunks[0];
    let help_subchunk = footer_subchunks[1];

    // Mode status
    let current_navigation_text = match app.current_screen {
        CurrentScreen::Main => Span::styled("Search Mode", Style::default().fg(Color::Green)),
        CurrentScreen::EditingCommand => {
            Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
        }
    }
    .to_owned();

    let mode_block = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::NONE));

    let help_block = Paragraph::new(Span::styled(
        "(Esc) quit | (Enter) to select command | Type character to search a command | (↑) move up | (↓) move down | (Backspace)/(Del): erase last character",
        Style::default().fg(Color::Yellow),
    ))
    .style(Style::new().bg(Color::Black))
    .left_aligned()
    .block(Block::default().borders(Borders::NONE));

    frame.render_widget(mode_block, mode_subchunk);
    frame.render_widget(help_block, help_subchunk);
}
