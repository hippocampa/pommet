use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
};

use super::{cpanel::ControlPanel, helptext::HelpText};

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

        let control_panel = ControlPanel::default();
        let helptext = HelpText {};

        control_panel.render(vertical_split[0], buf);
        helptext.render(vertical_split[1], buf);
    }
}
