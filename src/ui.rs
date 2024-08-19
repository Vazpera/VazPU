use ratatui::widgets::block::Title;
use ratatui::{
    layout::Alignment,
    layout::Layout,
    prelude::Constraint,
    prelude::Direction,
    prelude::Line,
    prelude::Style,
    style::{Color, Stylize},
    widgets,
    widgets::{Block, Paragraph, Widget},
    Frame,
};

use crate::app::*;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // The layout division
    // The acctual control display
    let controls = Title::default()
        .content(Line::from(vec![
            "<Q>:".yellow(),
            "Quit ".white(),
            "<LEFT>:".yellow(),
            "Decrement ".white(),
            "<RIGHT>:".yellow(),
            "Increment ".white(),
            "<SPACE>:".yellow(),
            "Change Counter".white(),
        ]))
        .position(widgets::block::Position::Bottom);

    match app.curr {
        CurrentScreen::Counter1 => Paragraph::new(format!(
            "This is a tui template.\n\
                        First Counter: {}",
            app.counter1
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title(controls)
                .title_alignment(Alignment::Center),
        )
        .style(Style::new().cyan().on_black())
        .centered()
        .render(frame.size(), frame.buffer_mut()),

        CurrentScreen::Counter2 => Paragraph::new(format!(
            "This is a tui template.\n\
                        Second Counter: {}",
            app.counter2
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .title(controls),
        )
        .style(Style::new().red().on_black())
        .centered()
        .render(frame.size(), frame.buffer_mut()),
    }
}
