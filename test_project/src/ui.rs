use ratatui::symbols;
use ratatui::widgets::block::{Position, Title};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    layout::Layout,
    layout::Rect,
    prelude::Constraint,
    prelude::Direction,
    prelude::Style,
    style::{Color, Stylize},
    text::Line,
    text::Span,
    widgets,
    widgets::{Bar, BarGroup, Block, BorderType, Dataset, Paragraph, Row, Widget},
    Frame,
};

use crate::app::*;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {

    // The controls
    let items = vec![
        "Q/CTRL+C/ESC-------QUIT",
        "SPACE----CHANGE COUNTER",
        "UP----INCRIMENT COUNTER",
        "DOWN--DECREMENT COUNTER",
    ];
    // The layout division
    let layout = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Fill(1),
            Constraint::Max((items[0].len() + 2).try_into().unwrap()),
        ],
    )
    .split(frame.size());
    // The acctual control display
    let controls = widgets::List::new(items)
        .block(Block::bordered().title(Title::from("CONTROLS").alignment(Alignment::Center)))
        .style(Style::new().bg(Color::Black))
        .render(layout[1], frame.buffer_mut());
    match app.curr {
        Current_Screen::Counter_1 => Paragraph::new(format!(
            "This is a tui template.\n\
                        
                        Counter: {}",
            app.counter_1
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
        )
        .style(Style::new().cyan().on_black())
        .centered()
        .render(layout[0], frame.buffer_mut()),

        Current_Screen::Counter_2 => Paragraph::new(format!(
            "This is a tui template.\n\
                        
                        Counter: {}",
            app.counter_2
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
        )
        .style(Style::new().red().on_black())
        .centered()
        .render(layout[0], frame.buffer_mut()),
    }
}
