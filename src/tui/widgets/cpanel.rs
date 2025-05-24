use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table, Widget},
};

use crate::plugins::Plugin;

#[derive(Default)]
pub struct ControlPanel<'a> {
    plugins: &'a [Box<dyn Plugin>],
    is_focused: bool,
    selected_index: usize, // Track which row is selected
}

impl<'a> ControlPanel<'a> {
    pub fn new(plugins: &'a [Box<dyn Plugin>], is_focused: bool, selected_index: usize) -> Self {
        Self {
            plugins,
            is_focused,
            selected_index,
        }
    }
}

impl<'a> Widget for ControlPanel<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let border_style = Style::default().fg(if self.is_focused {
            Color::Yellow
        } else {
            Color::Gray
        });

        let cpanel_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(Span::styled(
                " [1] Control Panel ",
                Style::default()
                    .fg(if self.is_focused {
                        Color::Yellow
                    } else {
                        Color::Gray
                    })
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center);

        let inner_area = cpanel_block.inner(area);

        let header = Row::new(vec![
            Cell::from(" Plugin Name ").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Cell::from(" Status ").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let rows = self.plugins.iter().enumerate().map(|(i, plugin)| {
            let status_text = plugin.status();
            let is_selected = self.is_focused && i == self.selected_index;

            // Add cursor indicator for selected row
            let name_prefix = if is_selected { "> " } else { "  " };

            Row::new(vec![
                Cell::from(format!("{}{}", name_prefix, plugin.get_name())).style(
                    Style::default().fg(if is_selected {
                        Color::White
                    } else {
                        Color::Gray
                    }),
                ),
                Cell::from(format!(" {} ", status_text)).style(Style::default().fg(
                    if status_text == "on" {
                        Color::Green
                    } else {
                        Color::Red
                    },
                )),
            ])
        });
        let table = Table::new(
            rows,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
        )
        .header(header)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
        .column_spacing(1)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .widths(&[Constraint::Percentage(70), Constraint::Percentage(30)])
        .column_spacing(2);

        cpanel_block.render(area, buf);
        table.render(inner_area, buf);
    }
}
