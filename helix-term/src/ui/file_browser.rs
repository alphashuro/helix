use helix_core::Position;
use helix_view::{
    graphics::Rect,
    input::Event,
    theme::{Color, Modifier, Style},
};
use tui::widgets::Borders;
use tui::widgets::StatefulWidget;
use tui::widgets::{Block, List, ListItem};
use tui::{buffer::Buffer as Surface, widgets::ListState};

use crate::{
    compositor::{Component, Context, EventResult},
    key,
};

pub struct FileBrowser {
    state: ListState,
    files: Vec<String>,
}

impl FileBrowser {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            files: vec![
                ("Item 1").to_string(),
                ("Item 2").to_string(),
                ("Item 3").to_string(),
            ],
        }
    }
}

impl Component for FileBrowser {
    fn handle_event(
        &mut self,
        event: &helix_view::input::Event,
        _ctx: &mut crate::compositor::Context,
    ) -> crate::compositor::EventResult {
        let start = 0;
        let end = self.files.len() - 1;

        match event {
            Event::Key(event) => {
                match (*event) {
                    key!('j') => {
                        let next = self.state.selected().map_or(start, |i| {
                            if i == end {
                                start
                            } else {
                                i + 1
                            }
                        });

                        self.state.select(Some(next));

                        EventResult::Consumed(None)
                    }
                    key!('k') => {
                        let prev =
                            self.state
                                .selected()
                                .map_or(end, |i| if i == start { end } else { i - 1 });

                        self.state.select(Some(prev));

                        EventResult::Consumed(None)
                    }
                    _ => EventResult::Ignored(None),
                }
            }
            _ => EventResult::Ignored(None),
        }
    }

    fn should_update(&self) -> bool {
        true
    }

    fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        if (self.files.len() > 0 && self.state.selected().is_none()) {
            self.state.select(Some(0))
        }

        let list = List::new(
            self.files
                .iter()
                .map(|f| ListItem::new(f.as_str()))
                .collect::<Vec<ListItem>>(),
        )
        .block(Block::default().title("Files").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">");

        let background = cx.editor.theme.get("ui.background");
        surface.clear_with(area, background);

        list.render(area, surface, &mut self.state);
    }

    fn cursor(
        &self,
        _area: Rect,
        _ctx: &helix_view::Editor,
    ) -> (
        Option<helix_core::Position>,
        helix_view::graphics::CursorKind,
    ) {
        (
            Some(Position::new(1, 0)),
            helix_view::graphics::CursorKind::Block,
        )
    }

    fn required_size(&mut self, _viewport: (u16, u16)) -> Option<(u16, u16)> {
        None
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn id(&self) -> Option<&'static str> {
        None
    }
}
