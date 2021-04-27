#![allow(dead_code, unused_imports)]

use iced::{
    button, scrollable, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, Radio, Row, Rule, Sandbox, Scrollable, Space, Text,
};

use sv_core::{config::Config, extractor::StockResult, util};

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    RefreshStock(Option<Vec<StockResult>>),
}

struct StockResultView {
    stock_result: StockResult,
}

impl StockResultView {
    pub fn view(&self) -> Element<Message> {
        let model = Text::new(&self.stock_result.model);
        let mut children = vec![model.into()];
        for d in &self.stock_result.data {
            let stock = Row::new()
                .push(Text::new(&d.name_html))
                .push(Text::new(&d.price.string));
            children.push(stock.into());
        }
        Column::with_children(children).into()
    }
}

struct StockResultUi {
    config: Config,
    stock_results: Option<Vec<StockResultView>>,
    controls: Controls,
    scroll_box: ScrollBox,
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
            .push(choose_theme)
            .push(self.controls.view());
        let mut contents = Vec::new();
        if let Some(s_viewers) = self.stock_results.as_ref() {
            for s in s_viewers {
                contents.push(s.view());
            }
        }
        let scroll_box = Column::with_children(contents);
        let main_content = Column::new().push(header).push(scroll_box);
        Container::new(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn update_stock_results(&mut self, stock_results: Vec<StockResult>) {
        let mut result = Vec::new();
        for stock_result in stock_results {
            result.push(StockResultView{stock_result});
        }
        self.stock_results = Some(result);
    }
}

#[derive(Default)]
struct Controls {
    refresh_button: button::State,
}

impl Controls {
    fn view(&mut self) -> Element<Message> {
        let label = Text::new("Refresh");
        Button::new(&mut self.refresh_button, label)
            .on_press(Message::RefreshStock(None))
            .into()
    }
}

pub struct App {
    theme: style::Theme,
    stock_result_ui: StockResultUi,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = util::get_config();
        let stock_result_ui = StockResultUi {
            config,
            stock_results: None,
            controls: Default::default(),
            scroll_box: Default::default(),
        };
        let app = Self {
            theme: Default::default(),
            stock_result_ui,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Stock Viewer")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            Message::RefreshStock(None) => {
                let stocks = self.stock_result_ui.config.stocks.clone();
                iced::Command::perform(refresh(stocks), |s| Message::RefreshStock(Some(s)))
            }
            Message::RefreshStock(Some(s)) => {
                self.stock_result_ui.update_stock_results(s);
                Command::none()
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

pub async fn refresh(stocks: Vec<sv_core::config::Stock>) -> Vec<StockResult> {
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
    stock_results
}
