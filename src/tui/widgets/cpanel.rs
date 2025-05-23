use ratatui::{
    layout::Alignment,
    widgets::{Block, Widget},
};

#[derive(Default)]
pub struct ControlPanel {
    is_selected: bool,
}

impl Widget for ControlPanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let cpanel_block = Block::bordered()
            .title("[1] Control Panel")
            .title_alignment(Alignment::Center);
        cpanel_block.render(area, buf);
    }
}
