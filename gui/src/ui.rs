#![allow(dead_code, unused_imports)]

use iced::{
    scrollable, Column, Container, Element, Length, Radio, Row, Rule, Sandbox, Scrollable, Space,
    Text,
};

use sv_core::extractor::StockResult;

use crate::style;

struct StockResultUi {
    stock_result: Option<StockResult>,
}

impl Default for StockResultUi {
    fn default() -> Self {
        Self { stock_result: None }
    }
}

impl StockResultUi {
    pub fn view(&mut self) -> Element<Message> {
        let header = Row::new()
            .push(Text::new("Stock Viewer"))
            .push(Text::new("Theme Selector"));
        let scroll_box = Row::new().push(Text::new("Stock goes here"));
        let main_content = Column::new().push(header).push(scroll_box);
        Container::new(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Default)]
pub struct App {
    theme: style::Theme,
    scroll_box: ScrollBox,
    stock_result_ui: StockResultUi,
}
#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Stock Viewer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Self { theme, scroll_box, .. } = self;

        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, option| {
                column.push(
                    Radio::new(
                        *option,
                        &format!("{:?}", option),
                        Some(*theme),
                        Message::ThemeChanged,
                    )
                    .style(*theme),
                )
            },
        );

        let content = Text::new("Some content that should wrap within the scrollable. Let's output a lot of short words, so that we'll make sure to see how wrapping works with these scrollbars.");

        let scrollable = Scrollable::new(&mut scroll_box.state)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(*theme)
            .push(content);

        let container = Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(*theme);

        let row = Row::new()
            .push(container)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(choose_theme)
            .push(Rule::horizontal(20).style(self.theme))
            .push(row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme);
        //.into();
        self.stock_result_ui.view()
    }
}

pub struct ScrollBox {
    state: scrollable::State,
}

impl Default for ScrollBox {
    fn default() -> Self {
        Self {
            state: scrollable::State::new(),
        }
    }
}
