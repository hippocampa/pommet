use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

use crate::plugins::Plugin;

#[derive(Default)]
pub struct ControlPanel<'a> {
    plugins: &'a [Box<dyn Plugin>],
    is_focused: bool,
}

impl<'a> ControlPanel<'a> {
    pub fn new(plugins: &'a [Box<dyn Plugin>], is_focused: bool) -> Self {
        Self {
            plugins,
            is_focused,
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

        // Create the outer block
        let cpanel_block = Block::bordered()
            .border_style(border_style)
            .title("[1] Control Panel")
            .title_alignment(Alignment::Center);

        // Calculate the inner area for the table
        let inner_area = cpanel_block.inner(area);

        // Create the table header
        let header = Row::new(vec![
            Cell::from("Plugin Name").style(Style::default().fg(Color::Green)),
            Cell::from("Status").style(Style::default().fg(Color::Green)),
        ]);

        // Create rows for each plugin
        let rows = self.plugins.iter().map(|plugin| {
            Row::new(vec![
                Cell::from(plugin.get_name().as_str()),
                Cell::from(plugin.status().as_str()).style(Style::default().fg(
                    if plugin.status() == "on" {
                        Color::Green
                    } else {
                        Color::Red
                    },
                )),
            ])
        });

        // Create the table
        let table = Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(header)
        .row_highlight_style(Style::default().bg(Color::DarkGray))
        .column_spacing(1);

        // Render the block and the table
        cpanel_block.render(area, buf);
        table.render(inner_area, buf);
    }
}
