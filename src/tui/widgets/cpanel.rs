use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Cell, Row, Widget},
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

        let table_header = ["plugin", "status"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let cpanel_block = Block::bordered()
            .border_style(border_style)
            .title("[1] Control Panel")
            .title_alignment(Alignment::Center);
        cpanel_block.render(area, buf);
    }
}
