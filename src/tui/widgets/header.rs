use ratatui::{
    layout::Alignment,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

static PKG_INFO: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

pub struct Header<'a> {
    title: Option<&'a str>,
    text: &'a str,
    description: Option<&'a str>,
}

impl<'a> Header<'a> {
    pub fn new() -> Header<'a> {
        Header {
            title: None,
            text: PKG_INFO,
            description: Some(
                "php local mini development toolkit -- https://github.com/hippocampa/pommet",
            ),
        }
    }
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = match self.title {
            Some(title) => Block::bordered()
                .title(title)
                .title_alignment(Alignment::Center),
            None => Block::bordered(),
        };

        let inner_area = block.inner(area);
        block.render(area, buf);

        let text_content = if let Some(desc) = self.description {
            format!("{}\n\n{}", self.text, desc)
        } else {
            self.text.to_string()
        };

        Paragraph::new(Text::from(text_content))
            .alignment(Alignment::Center)
            .render(inner_area, buf);
    }
}
