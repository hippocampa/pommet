use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default)]
pub struct HelpText {}

impl Widget for HelpText {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let helptext_block = Block::default();
        let inner_area = helptext_block.inner(area);
        helptext_block.render(area, buf);

        let help_text = Text::from(vec![
            Line::from(vec![
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(": Quit  "),
                Span::styled("1-2", Style::default().fg(Color::Yellow)),
                Span::raw(": Select Pane  "),
                Span::styled("Space", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle Menu"),
            ]),
            Line::from(vec![
                Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Navigate  "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": Select Item  "),
                Span::styled("?", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle Help"),
            ]),
        ]);

        Paragraph::new(help_text)
            .alignment(Alignment::Left)
            .render(inner_area, buf);
    }
}
