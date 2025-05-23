use ratatui::widgets::{Block, Borders, Widget};

pub struct RightPanel;

impl Widget for RightPanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let top_block = Block::default().borders(Borders::ALL).title("Top Panel");
        top_block.render(area, buf);
    }
}
