use iced::widget::{button, column, container, image, row, scrollable, text, text_input};
use iced::{Border, Color, Element, Font, Length, Shadow, Task, Theme, Vector};
use serde::Deserialize;
use iced_fonts::{LUCIDE_FONT_BYTES, lucide::*};

// api
const API_KEY: &str = "";

#[derive(Default)]
struct Counter {
    query: String,
    movies: Vec<Movie>,
    posters: Vec<Option<image::Handle>>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    QueryChanged(String),
    Search,
    Searched(Result<ApiResponse, String>),
    PosterLoaded(usize, Result<Vec<u8>, String>),
}

#[derive(Debug, Clone, Deserialize)]
struct Movie {
    #[allow(dead_code)]
    id: i64,
    title: String,
    #[serde(default)]
    overview: String,
    #[serde(default)]
    release_date: String,
    #[serde(default)]
    vote_average: f64,
    #[serde(default)]
    poster_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ApiResponse {
    #[allow(dead_code)]
    page: i64,
    results: Vec<Movie>,
    #[allow(dead_code)]
    total_pages: i64,
    #[allow(dead_code)]
    total_results: i64,
}

impl Counter {
    pub fn view(&self) -> Element<'_, Message> {
        let title = text("⚡ Movie Info ⚡")
            .size(34)
            .font(Font::MONOSPACE)
            .style(|_theme: &Theme| text::Style {
                color: Some(Color::from_rgb8(0, 255, 200)),
            });

        let search_input = text_input("Movie name...", &self.query)
            .on_input(Message::QueryChanged)
            .on_submit(Message::Search)
            .padding(12)
            .size(18);

        let search_btn = button(text("🔎 Search").size(18).font(Font::MONOSPACE))
            .padding([12, 22])
            .on_press(Message::Search)
            .style(neon_button_style);

        let search_row = row![search_input, search_btn].spacing(12);

        let mut results_column = column![].spacing(18);

        if let Some(err) = &self.error {
            results_column = results_column.push(
                text(format!("⚠ {err}"))
                    .size(16)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(Color::from_rgb8(255, 90, 90)),
                    }),
            );
        }

        for (i, movie) in self.movies.iter().enumerate() {
            let poster_widget: Element<'_, Message> =
                match self.posters.get(i).and_then(|p| p.clone()) {
                    Some(handle) => image(handle)
                        .width(Length::Fixed(110.0))
                        .height(Length::Fixed(165.0))
                        .into(),
                    None => container(text("..."))
                        .width(Length::Fixed(110.0))
                        .height(Length::Fixed(165.0))
                        .align_x(iced::alignment::Horizontal::Center)
                        .align_y(iced::alignment::Vertical::Center)
                        .style(|_theme: &Theme| container::Style {
                            background: Some(Color::from_rgb8(30, 10, 50).into()),
                            border: Border {
                                color: Color::from_rgb8(255, 0, 200),
                                width: 1.5,
                                radius: 10.0.into(),
                            },
                            ..container::Style::default()
                        })
                        .into(),
                };

            let year = movie
                .release_date
                .split('-')
                .next()
                .unwrap_or("—")
                .to_string();

            let card_text = column![
                text(movie.title.clone())
                    .size(20)
                    .font(Font::MONOSPACE)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(Color::from_rgb8(0, 255, 200)),
                    }),
                text(format!("{year} ★ {:.1}", movie.vote_average))
              
                    .size(14)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(Color::from_rgb8(255, 200, 60)),
                    }),
                text(if movie.overview.is_empty() {
                    "No description available.".to_string()
                } else {
                    movie.overview.clone()
                })
                .size(14)
                .style(|_theme: &Theme| text::Style {
                    color: Some(Color::from_rgb8(220, 220, 230)),
                }),
            ]
            .spacing(8)
            .width(Length::Fill);

            let card_row = row![poster_widget, card_text].spacing(18);

            let card = container(card_row)
                .padding(16)
                .width(Length::Fill)
                .style(|_theme: &Theme| container::Style {
                    background: Some(Color::from_rgb8(18, 0, 36).into()),
                    border: Border {
                        color: Color::from_rgb8(255, 0, 200),
                        width: 2.0,
                        radius: 16.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::from_rgba8(255, 0, 200, 0.4),
                        offset: Vector::new(0.0, 2.0),
                        blur_radius: 10.0,
                    },
                    ..container::Style::default()
                });

            results_column = results_column.push(card);
        }

        let results = scrollable(results_column.padding(4)).height(Length::Fill);

        let content = column![title, search_row, results]
            .spacing(24)
            .padding(30)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::from_rgb8(5, 5, 15).into()),
                ..container::Style::default()
            })
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QueryChanged(value) => {
                self.query = value;
                Task::none()
            }

            Message::Search => {
                if self.query.trim().is_empty() {
                    return Task::none();
                }
                self.movies.clear();
                self.posters.clear();
                self.error = None;
                Task::perform(search_movies(self.query.clone()), Message::Searched)
            }

            Message::Searched(Ok(response)) => {
                self.movies = response.results;
                self.posters = vec![None; self.movies.len()];
                self.error = None;

                let mut tasks = Vec::new();
                for (i, movie) in self.movies.iter().enumerate() {
                    if let Some(path) = &movie.poster_path {
                        let url = format!("https://image.tmdb.org/t/p/w500{path}");
                        tasks.push(Task::perform(fetch_poster(url), move |result| {
                            Message::PosterLoaded(i, result)
                        }));
                    }
                }
                Task::batch(tasks)
            }

            Message::Searched(Err(err)) => {
                self.error = Some(format!("Search failed: {err}"));
                Task::none()
            }

            Message::PosterLoaded(index, Ok(bytes)) => {
                if let Some(slot) = self.posters.get_mut(index) {
                    *slot = Some(image::Handle::from_bytes(bytes));
                }
                Task::none()
            }

            Message::PosterLoaded(_index, Err(_err)) => Task::none(),
        }
    }
}

fn neon_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = Color::from_rgb8(255, 60, 180);
    let hover = Color::from_rgb8(0, 255, 220);

    let bg = match status {
        button::Status::Hovered => hover,
        button::Status::Pressed => Color::from_rgb8(255, 255, 0),
        _ => base,
    };
    button::Style {
        background: Some(bg.into()),
        text_color: Color::BLACK,
        border: Border {
            color: Color::WHITE,
            width: 2.0,
            radius: 30.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba8(0, 0, 0, 0.4),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        },
        ..button::Style::default()
    }
}

async fn search_movies(query: String) -> Result<ApiResponse, String> {
    let querystring = [
        ("query", query.as_str()),
        ("include_adult", "false"),
        ("language", "en-US"),
        ("page", "1"),
    ];

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.themoviedb.org/3/search/movie")
        .query(&querystring)
        .header("accept", "application/json")
        .header("Authorization", format!("Bearer {API_KEY}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    response
        .json::<ApiResponse>()
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_poster(url: String) -> Result<Vec<u8>, String> {
    let bytes = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    Ok(bytes.to_vec())
}

fn main() -> iced::Result {
    iced::application("Movie Info", Counter::update, Counter::view).run()

    
}
