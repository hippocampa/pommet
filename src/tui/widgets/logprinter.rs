use ratatui::{
    layout::Alignment,
    widgets::{Block, Widget},
};

#[derive(Default)]
pub struct LogPrinter {
    is_selected: bool,
}

impl Widget for LogPrinter {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let logprinter_block = Block::bordered()
            .title("[2] Log")
            .title_alignment(Alignment::Center);
        logprinter_block.render(area, buf);
    }
}
