use crate::app::github_api::GithubApiClient;
use namui::prelude::*;
use namui_prebuilt::simple_rect;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

const CLIENT_ID: &str = "Iv1.7b68d8d6f7b401df";
const CLIENT_SECRET: &str = "350c026f799464cc1f0cb080325ba666d758e06d";

pub struct AuthenticationProps {
    pub wh: Wh<Px>,
}

pub struct Authentication {
    authentication_state: AuthenticationState,
}

impl Authentication {
    pub fn new() -> Self {
        Self {
            authentication_state: AuthenticationState::Idle,
        }
    }
}

impl Authentication {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<AuthenticationEvent>() {
            match event {
                AuthenticationEvent::LoginButtonClicked => {
                    self.authentication_state = AuthenticationState::WaitForOauthCallback;
                    open_oauth_page();
                }
                AuthenticationEvent::LoginFailed => {
                    self.authentication_state = AuthenticationState::LoginFailed;
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::AnimationFrame
                | NamuiEvent::MouseDown(_)
                | NamuiEvent::MouseUp(_)
                | NamuiEvent::MouseMove(_)
                | NamuiEvent::KeyDown(_)
                | NamuiEvent::KeyUp(_)
                | NamuiEvent::ScreenResize(_)
                | NamuiEvent::Wheel(_) => {}
                NamuiEvent::DeepLinkOpened(DeepLinkOpenedEvent { url }) => {
                    let url = Url::parse(url).unwrap();
                    let code = url.query_pairs().find(|(key, _)| key == "code");
                    if let Some((_, code)) = code {
                        self.login_with_oauth_code(code.to_string());
                    }
                }
            }
        }
    }

    pub fn render(&self, props: &AuthenticationProps) -> RenderingTree {
        let window_center = Rect::from_xy_wh(Xy::zero(), props.wh).center();
        let access_code_input_width = props.wh.width;
        let instruction_text = self.get_instruction_text();
        render([
            render_background(props.wh),
            render_instruction_text(&window_center, instruction_text),
            render_login_button(window_center, access_code_input_width),
        ])
    }

    fn get_instruction_text(&self) -> String {
        match self.authentication_state {
            AuthenticationState::Idle => "Click button to login with github".to_string(),
            AuthenticationState::WaitForOauthCallback => "Login from newly opened page".to_string(),
            AuthenticationState::LoginInProgress => "Logging in...".to_string(),
            AuthenticationState::LoginFailed => "Login failed.".to_string(),
        }
    }

    fn login_with_oauth_code(&mut self, code: String) {
        self.authentication_state = AuthenticationState::LoginInProgress;
        spawn_local(async move {
            match GithubApiClient::get_access_token_with_oauth_code(
                CLIENT_ID,
                CLIENT_SECRET,
                code.as_str(),
            )
            .await
            {
                Ok(token) => {
                    let client = create_github_api_client(token);
                    match client.get_user_id().await {
                        Ok(user_id) => {
                            namui::event::send(AuthenticationEvent::LoginSucceeded {
                                user_id,
                                github_api_client: Arc::new(client),
                            });
                        }
                        Err(error) => {
                            namui::log!("fail to get user id: {error:#?}");
                            namui::event::send(AuthenticationEvent::LoginFailed);
                        }
                    }
                }
                Err(error) => {
                    namui::log!("fail to get user id: {error:#?}");
                    namui::event::send(AuthenticationEvent::LoginFailed)
                }
            };
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
    simple_rect(wh, Color::TRANSPARENT, 0.0.px(), Color::grayscale_f01(0.3))
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

fn render_login_button(center: Xy<Px>, width: Px) -> RenderingTree {
    const HEIGHT: Px = px(36.0);
    const STROKE_WIDTH: Px = px(4.0);
    const FONT_SIZE: IntPx = int_px(24);
    const MARGIN: Px = px(8.0);
    let login_button_rect = Rect::Xywh {
        x: MARGIN,
        y: center.y,
        width: width - 2 * MARGIN,
        height: HEIGHT,
    };
    let login_button_rect_center = login_button_rect.center();
    render([
        translate(
            login_button_rect.x(),
            login_button_rect.y(),
            simple_rect(
                login_button_rect.wh(),
                Color::grayscale_f01(0.6),
                STROKE_WIDTH,
                Color::grayscale_f01(0.6),
            ),
        )
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
        user_id: u32,
        github_api_client: Arc<GithubApiClient>,
    },
}

enum AuthenticationState {
    Idle,
    WaitForOauthCallback,
    LoginInProgress,
    LoginFailed,
}

fn open_oauth_page() {
    let url = format!("https://github.com/login/oauth/authorize?client_id={CLIENT_ID}");
    open_external(url.as_str());
}
