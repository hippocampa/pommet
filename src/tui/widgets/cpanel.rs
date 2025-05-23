use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Widget},
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
        let cpanel_block = Block::bordered()
            .border_style(border_style)
            .title("[1] Control Panel")
            .title_alignment(Alignment::Center);
        cpanel_block.render(area, buf);
    }
}
