use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Widget},
};

use crate::plugins::{Plugin, PluginStatus};

pub struct Dashboard<'a> {
    pub plugins: &'a [Box<dyn Plugin>],
    pub selected_index: usize,
}

impl<'a> Widget for Dashboard<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(6),
                Constraint::Min(8),
                Constraint::Length(4),
            ])
            .split(area);

        self.render_header(chunks[0], buf);

        self.render_plugin_table(chunks[1], buf);

        self.render_instructions(chunks[2], buf);
    }
}

impl<'a> Dashboard<'a> {
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            }));
        let left_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(147, 112, 219)))
            .title(" Project Info ")
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(147, 112, 219))
                    .bold(),
            );
        let project_info = vec![
            Line::from(vec![Span::styled(
                "pommet",
                Style::default().fg(Color::Rgb(155, 89, 182)).bold(),
            )]),
            Line::from(vec![Span::styled(
                "v0.1.0",
                Style::default().fg(Color::Rgb(46, 204, 113)),
            )]),
        ];
        Paragraph::new(Text::from(project_info))
            .block(left_block)
            .alignment(Alignment::Center)
            .render(header_chunks[0], buf);
        let right_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(72, 61, 139)))
            .title(" Details ")
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(72, 61, 139))
                    .bold(),
            );
        let details_info = vec![
            Line::from(vec![Span::styled(
                "Windows Dev Stack Manager",
                Style::default().fg(Color::Rgb(230, 230, 250)),
            )]),
            Line::from(vec![Span::styled(
                "github.com/your-repo/pommet",
                Style::default().fg(Color::Rgb(135, 206, 235)).underlined(),
            )]),
        ];

        Paragraph::new(Text::from(details_info))
            .block(right_block)
            .alignment(Alignment::Center)
            .render(header_chunks[1], buf);
    }
    fn render_plugin_table(&self, area: Rect, buf: &mut Buffer) {
        let toggleable_plugins: Vec<(usize, &Box<dyn Plugin>)> = self
            .plugins
            .iter()
            .enumerate()
            .filter(|(_, plugin)| plugin.is_toggleable())
            .collect();

        if toggleable_plugins.is_empty() {
            let empty_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title("No Toggleable Plugins")
                .title_style(Style::default().fg(Color::Red).bold());

            Paragraph::new("No toggleable plugins available")
                .block(empty_block)
                .alignment(Alignment::Center)
                .render(area, buf);
            return;
        }
        let rows: Vec<Row> = toggleable_plugins
            .iter()
            .enumerate()
            .map(|(display_index, (_, plugin))| {
                let status_text = match plugin.status() {
                    PluginStatus::On => "RUNNING",
                    PluginStatus::Off => "STOPPED",
                };
                let status_style = match plugin.status() {
                    PluginStatus::On => Style::default().fg(Color::Rgb(46, 204, 113)).bold(),
                    PluginStatus::Off => Style::default().fg(Color::Rgb(231, 76, 60)),
                };
                let name_style = if display_index == self.selected_index {
                    Style::default()
                        .fg(Color::Rgb(255, 255, 255))
                        .bg(Color::Rgb(52, 152, 219))
                        .bold()
                } else {
                    Style::default().fg(Color::Rgb(236, 240, 241))
                };

                let status_cell_style = if display_index == self.selected_index {
                    Style::default()
                        .fg(Color::Rgb(255, 255, 255))
                        .bg(Color::Rgb(52, 152, 219))
                        .add_modifier(Modifier::BOLD)
                } else {
                    status_style
                };

                Row::new(vec![
                    Cell::from(plugin.name().clone()).style(name_style),
                    Cell::from(status_text).style(status_cell_style),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            vec![Constraint::Percentage(70), Constraint::Percentage(30)],
        )
        .header(
            Row::new(vec!["Plugin Name", "Status"])
                .style(Style::default().fg(Color::Rgb(155, 89, 182)).bold())
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(52, 152, 219)))
                .title(" Plugins (↑↓/jk: navigate, Space: toggle) ")
                .title_style(Style::default().fg(Color::Rgb(241, 196, 15)).bold()),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::Rgb(52, 152, 219))
                .fg(Color::Rgb(255, 255, 255)),
        )
        .highlight_symbol("► ");

        Widget::render(table, area, buf);
    }

    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        let instructions_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(155, 89, 182)))
            .title(" Quick Guide ")
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(155, 89, 182))
                    .bold(),
            );
        let instructions_text = vec![
            Line::from(vec![
                Span::styled(
                    "Navigation: ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).bold(),
                ),
                Span::styled("↑↓ or j/k", Style::default().fg(Color::Rgb(236, 240, 241))),
                Span::styled(" • ", Style::default().fg(Color::Rgb(155, 89, 182))),
                Span::styled(
                    "Toggle: ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).bold(),
                ),
                Span::styled("Space", Style::default().fg(Color::Rgb(236, 240, 241))),
                Span::styled(" • ", Style::default().fg(Color::Rgb(155, 89, 182))),
                Span::styled(
                    "Quit: ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).bold(),
                ),
                Span::styled("q", Style::default().fg(Color::Rgb(236, 240, 241))),
            ]),
            Line::from(vec![
                Span::styled(
                    "Access: ",
                    Style::default().fg(Color::Rgb(46, 204, 113)).bold(),
                ),
                Span::styled("Apache at ", Style::default().fg(Color::Rgb(230, 230, 250))),
                Span::styled(
                    "localhost",
                    Style::default().fg(Color::Rgb(135, 206, 235)).underlined(),
                ),
                Span::styled(" • ", Style::default().fg(Color::Rgb(155, 89, 182))),
                Span::styled(
                    "phpMyAdmin at ",
                    Style::default().fg(Color::Rgb(230, 230, 250)),
                ),
                Span::styled(
                    "localhost/phpmyadmin",
                    Style::default().fg(Color::Rgb(135, 206, 235)).underlined(),
                ),
            ]),
        ];

        Paragraph::new(Text::from(instructions_text))
            .block(instructions_block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
