use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
};

use crate::plugins::Plugin;

use super::{cpanel::ControlPanel, helptext::HelpText};

pub struct LeftPanel<'a> {
    plugins: &'a [Box<dyn Plugin>],
    is_focused: bool,
}

impl<'a> LeftPanel<'a> {
    pub fn new(plugins: &'a [Box<dyn Plugin>], is_focused: bool) -> Self {
        Self {
            plugins,
            is_focused,
        }
    }
}
impl<'a> Widget for LeftPanel<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let control_panel = ControlPanel::new(&self.plugins, self.is_focused);
        let helptext = HelpText {};

        control_panel.render(vertical_split[0], buf);
        helptext.render(vertical_split[1], buf);
    }
}
