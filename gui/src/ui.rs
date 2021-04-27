#![allow(dead_code, unused_imports)]

use iced::{
    scrollable, Application, Clipboard, Column, Command, Container, Element, Length, Radio, Row,
    Rule, Sandbox, Scrollable, Space, Text,
};

use sv_core::{config::Config, extractor::StockResult, util};

use crate::style;

struct StockResultUi {
    config: Option<Config>,
    stock_result: Option<StockResult>,
}

impl Default for StockResultUi {
    fn default() -> Self {
        Self {
            config: None,
            stock_result: None,
        }
    }
}

impl StockResultUi {
    pub fn view(&mut self, theme: &style::Theme) -> Element<Message> {
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
        let header = Row::new()
            .push(Text::new("Stock Viewer"))
            .push(choose_theme);
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
    Refreshed,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut app = Self::default();
        let config = util::get_config();
        app.stock_result_ui.config = Some(config);
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Stock Viewer")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                let stocks = self.stock_result_ui.config.as_ref().unwrap().stocks.clone();
                iced::Command::perform(refresh(stocks), |_| Message::Refreshed)
            }
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Self { theme, .. } = self;
        self.stock_result_ui.view(&theme)
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

pub async fn refresh(stocks: Vec<sv_core::config::Stock>) {
    log::info!("Test");
    let mut stock_results: Vec<StockResult> = Vec::new();
    let mut tasks = Vec::new();
    for c in stocks {
        let query = util::Query {
            model: c.name.to_string(),
            url: c.sites[0].url.clone(),
        };
        tasks.push(tokio::spawn(async move {
            util::get_stock_result(query).await.unwrap()
        }));
    }
    for task in tasks {
        stock_results.push(task.await.unwrap());
    }
}
