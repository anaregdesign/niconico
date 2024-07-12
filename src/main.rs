use std::env;
use std::rc::Rc;

use nannou::prelude::*;
use nannou::text::Font;

use model::MessageScreen;
use repository::RedisMessageRepository;

mod model;
mod repository;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    message_screen: MessageScreen,
    message_repository: RedisMessageRepository,
}

fn model(_app: &App) -> Model {
    let font = Vec::from(include_bytes!("../assets/BIZUDPGothic-Bold.ttf") as &[u8]);
    let font = Font::from_bytes(font).unwrap();
    let message_screen = MessageScreen::new(font);

    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    let redis_password =
        env::var("REDIS_PASSWORD").expect("missing environment variable REDIS_PASSWORD");
    let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_host_name);
    let redis_client = redis::Client::open(redis_conn_url).expect("failed to open redis client");
    let repo = repository::RedisMessageRepository {
        client: Rc::new(redis_client),
    };
    Model {
        message_screen,
        message_repository: repo,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    match _event {
        Event::WindowEvent { id: _id, simple: window_event } => {
            if let Some(KeyPressed(key)) = window_event {
                match key {
                    Key::Up => _model.message_screen.add_message("こんにちは", _app),
                    Key::Return => {
                        let message = _model.message_repository.get_random();
                        _model.message_screen.add_message(&message, _app);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.message_screen.update(_app);
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    _model.message_screen.draw(&draw, 48.0);

    draw.to_frame(_app, &_frame).unwrap();
}
