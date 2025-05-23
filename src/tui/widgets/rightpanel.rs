use ratatui::widgets::{Block, Borders, Widget};

use super::logprinter::LogPrinter;

pub struct RightPanel;

impl Widget for RightPanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let logprinter = LogPrinter::default();
        logprinter.render(area, buf);
    }
}
