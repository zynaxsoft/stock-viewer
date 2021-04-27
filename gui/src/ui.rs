#![allow(dead_code, unused_imports)]

use iced::{
    button, scrollable, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    HorizontalAlignment, Image, Length, Radio, Row, Rule, Sandbox, Scrollable, Space, Text,
};

use sv_core::{config::Config, extractor::StockResult, util};

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    RefreshStock(Option<Vec<StockResult>>),
}

struct StockPage {
    stock_result: StockResult,
}

impl StockPage {
    pub fn view(&self) -> Option<Element<Message>> {
        if self.stock_result.data.is_empty() {
            return None;
        }
        let model = Text::new(&self.stock_result.model).size(30);
        let mut children = vec![model.into()];
        for (i, d) in self.stock_result.data.iter().enumerate() {
            let product_name = &d.name_html[6..].replace("</span>", "");
            let mut product_text = Text::new(product_name).width(Length::Units(800));
            let mut price = Text::new(&d.price.string);
            if i == 0 {
                product_text = product_text.color([89.0 / 255.0, 115.0 / 255.0, 91.0 / 255.0]);
                price = price.color([89.0 / 255.0, 115.0 / 255.0, 91.0 / 255.0]);
            }
            let stock = Row::new()
                .push(product_text)
                .push(Space::with_width(Length::Fill))
                .push(price);
            children.push(stock.into());
        }
        Some(Column::with_children(children).into())
    }
}

struct StockResultUi {
    config: Config,
    stock_pages: Option<Vec<StockPage>>,
    controls: Controls,
    scroll_state: scrollable::State,
}

impl StockResultUi {
    pub fn view(&mut self, theme: &style::Theme) -> Element<Message> {
        let choose_theme =
            style::Theme::ALL
                .iter()
                .fold(Column::new().spacing(10), |column, option| {
                    column.push(
                        Radio::new(
                            *option,
                            &format!("{:?}", option),
                            Some(*theme),
                            Message::ThemeChanged,
                        )
                        .style(*theme),
                    )
                });
        let header = Row::new()
            .spacing(20)
            .padding(10)
            .push(Text::new("Stock Viewer").color([1.0, 0.2, 0.2]).size(40))
            .push(Space::with_width(Length::Fill))
            .push(self.controls.view())
            .push(choose_theme)
            .align_items(Align::Center);

        let mut scroll_box = Scrollable::new(&mut self.scroll_state).padding(40);
        if let Some(s_viewers) = self.stock_pages.as_ref() {
            for s in s_viewers {
                if let Some(content) = s.view() {
                    scroll_box = scroll_box
                        .push(content)
                        .push(Space::with_height(Length::Units(10)));
                }
            }
        }
        let main_content = Column::new()
            .push(header)
            .push(scroll_box)
            .push(Space::with_height(Length::Fill));
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
            result.push(StockPage { stock_result });
        }
        self.stock_pages = Some(result);
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
            stock_pages: None,
            controls: Default::default(),
            scroll_state: Default::default(),
        };
        let app = Self {
            theme: Default::default(),
            stock_result_ui,
        };
        (
            app,
            Command::perform(async {}, |_| Message::RefreshStock(None)),
        )
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
                Command::perform(refresh(stocks), |s| Message::RefreshStock(Some(s)))
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
