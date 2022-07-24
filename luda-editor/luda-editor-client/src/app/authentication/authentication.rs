use crate::app::github_api::GithubApiClient;
use namui::prelude::*;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

pub struct AuthenticationProps {
    pub wh: Wh<Px>,
}

pub struct Authentication {
    access_token_input: TextInput,
    access_token: String,
    authentication_state: AuthenticationState,
}

impl Authentication {
    pub fn new() -> Self {
        Self {
            access_token_input: TextInput::new(),
            access_token: String::new(),
            authentication_state: AuthenticationState::Idle,
        }
    }
}

impl Authentication {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::TextUpdated(update) => {
                    if update.id == self.access_token_input.get_id() {
                        self.access_token = update.text.clone();
                    }
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<AuthenticationEvent>() {
            match event {
                AuthenticationEvent::LoginButtonClicked => {
                    self.login();
                }
                AuthenticationEvent::LoginFailed => {
                    self.authentication_state = AuthenticationState::LoginFailed;
                }
                _ => {}
            }
        }
        self.access_token_input.update(event);
    }

    pub fn render(&self, props: &AuthenticationProps) -> RenderingTree {
        let window_center = Rect::from_xy_wh(Xy::zero(), props.wh).center();
        let access_code_input_width = props.wh.width;
        let instruction_text = self.get_instruction_text();
        render([
            render_background(props.wh),
            render_instruction_text(&window_center, instruction_text),
            render_access_code_input(
                &self.access_token,
                &self.access_token_input,
                window_center,
                access_code_input_width,
            ),
        ])
    }

    fn get_instruction_text(&self) -> String {
        match self.authentication_state {
            AuthenticationState::Idle => "Enter your GitHub access token".to_string(),
            AuthenticationState::LoginInProgress => "Logging in...".to_string(),
            AuthenticationState::LoginFailed => "Login failed. Check your token".to_string(),
        }
    }

    fn login(&mut self) {
        self.authentication_state = AuthenticationState::LoginInProgress;
        let token = self.access_token.clone();
        spawn_local(async move {
            let client = create_github_api_client(token);
            match client.validate_token().await {
                Ok(_) => {
                    namui::event::send(AuthenticationEvent::LoginSucceeded {
                        github_api_client: Arc::new(client),
                    });
                }
                Err(_) => {
                    namui::event::send(AuthenticationEvent::LoginFailed);
                }
            }
        })
    }
}

fn create_github_api_client(access_token: String) -> GithubApiClient {
    const BASE_URL: &str = "https://api.github.com";
    const OWNER: &str = "bigfoodK";
    const REPO: &str = "api-test";
    GithubApiClient::new(
        access_token,
        BASE_URL.to_string(),
        OWNER.to_string(),
        REPO.to_string(),
    )
}

fn render_background(wh: Wh<Px>) -> RenderingTree {
    rect(RectParam {
        rect: Rect::from_xy_wh(Xy::zero(), wh),
        style: RectStyle {
            stroke: None,
            fill: Some(RectFill {
                color: Color::grayscale_f01(0.3),
            }),
            round: None,
        },
    })
}

fn render_instruction_text(center: &Xy<Px>, instruction_text: String) -> RenderingTree {
    const INSTRUCTION_TEXT_HEIGHT: Px = px(48.0);
    namui::text(TextParam {
        text: instruction_text,
        x: center.x,
        y: center.y,
        align: TextAlign::Center,
        baseline: TextBaseline::Bottom,
        font_type: FontType {
            serif: false,
            size: INSTRUCTION_TEXT_HEIGHT.into(),
            language: Language::Ko,
            font_weight: FontWeight::REGULAR,
        },
        style: TextStyle {
            border: None,
            drop_shadow: None,
            color: Color::WHITE,
            background: None,
        },
    })
}

fn render_access_code_input(
    access_code: &String,
    access_code_input: &TextInput,
    center: Xy<Px>,
    width: Px,
) -> RenderingTree {
    const HEIGHT: Px = px(36.0);
    const FONT_SIZE: IntPx = int_px(24);
    const STROKE_WIDTH: Px = px(4.0);
    const BUTTON_WIDTH: Px = px(128.0);
    const MARGIN: Px = px(8.0);
    let access_code_input_rect = Rect::Xywh {
        x: px(0.0),
        y: center.y,
        width: width - MARGIN - BUTTON_WIDTH,
        height: HEIGHT,
    };
    let login_button_rect = Rect::Xywh {
        x: access_code_input_rect.x() + access_code_input_rect.width() + MARGIN,
        y: center.y,
        width: BUTTON_WIDTH,
        height: HEIGHT,
    };
    let login_button_rect_center = login_button_rect.center();
    let access_code_input_rect_center = access_code_input_rect.center();
    render([
        access_code_input
            .render(text_input::Props {
                rect_param: RectParam {
                    rect: access_code_input_rect,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::grayscale_f01(0.6),
                            width: STROKE_WIDTH,
                            border_position: BorderPosition::Middle,
                        }),
                        fill: None,
                        round: None,
                    },
                },
                text_param: TextParam {
                    text: access_code.clone(),
                    x: access_code_input_rect_center.x,
                    y: access_code_input_rect_center.y,
                    align: TextAlign::Center,
                    baseline: TextBaseline::Middle,
                    font_type: FontType {
                        serif: false,
                        size: FONT_SIZE,
                        language: Language::Ko,
                        font_weight: FontWeight::REGULAR,
                    },
                    style: TextStyle {
                        border: None,
                        drop_shadow: None,
                        color: Color::WHITE,
                        background: None,
                    },
                },
            })
            .with_mouse_cursor(MouseCursor::Text),
        rect(RectParam {
            rect: login_button_rect,
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::grayscale_f01(0.6),
                    width: STROKE_WIDTH,
                    border_position: BorderPosition::Middle,
                }),
                fill: Some(RectFill {
                    color: Color::grayscale_f01(0.6),
                }),
                round: None,
            },
        })
        .attach_event(|builder| {
            builder.on_mouse_down_in(|_event| {
                event::send(AuthenticationEvent::LoginButtonClicked);
            });
        })
        .with_mouse_cursor(MouseCursor::Pointer),
        namui::text(TextParam {
            text: "Login".to_string(),
            x: login_button_rect_center.x,
            y: login_button_rect_center.y,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                serif: false,
                size: FONT_SIZE,
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: Color::WHITE,
                background: None,
            },
        }),
    ])
}

pub enum AuthenticationEvent {
    LoginButtonClicked,
    LoginFailed,
    LoginSucceeded {
        github_api_client: Arc<GithubApiClient>,
    },
}

enum AuthenticationState {
    Idle,
    LoginInProgress,
    LoginFailed,
}
