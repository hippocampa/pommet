use ratatui::{
    layout::{Alignment, Constraint, Rect},
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
    pub fn new(plugins: &'a [Box<dyn Plugin>], is_focused: bool) -> Self {
        Self {
            plugins,
            is_focused,
            selected_index: 0, // Default: first item selected
        }
    }

    // Method to set selected index
    pub fn select(&mut self, index: usize) {
        if !self.plugins.is_empty() {
            self.selected_index = index.min(self.plugins.len() - 1);
        }
    }

    // Method to move selection up
    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    // Method to move selection down
    pub fn next(&mut self) {
        if !self.plugins.is_empty() && self.selected_index < self.plugins.len() - 1 {
            self.selected_index += 1;
        }
    }

    // Method to toggle the selected plugin
    pub fn toggle_selected(&mut self) {
        // This would require plugins to be mutable - would need implementation elsewhere
        // For now, this is just a placeholder for the interface
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

        // Create the outer block with a more decorative border
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

        // Calculate the inner area for the table
        let inner_area = cpanel_block.inner(area);

        // Create the table header with improved styling
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

        // Create rows for each plugin with indicators for selection
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

        // Create the enhanced table
        let table = Table::new(
            rows,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
        )
        .header(header)
        .highlight_style(
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

        // Render the block and the table
        cpanel_block.render(area, buf);
        table.render(inner_area, buf);
    }
}
