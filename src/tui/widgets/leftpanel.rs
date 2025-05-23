use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
};

use super::cpanel::ControlPanel;

pub struct LeftPanel;
impl Widget for LeftPanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let top_block = Block::default().borders(Borders::ALL).title("Top Panel");
        let bottom_block = Block::default().borders(Borders::ALL).title("Bottom Panel");

        top_block.render(vertical_split[0], buf);
        bottom_block.render(vertical_split[1], buf);
    }
}
