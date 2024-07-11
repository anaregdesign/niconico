use nannou::prelude::*;

use model::MessageScreen;

mod model;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    message_screen: MessageScreen,
}

fn model(_app: &App) -> Model {
    let w = _app.window_rect().x;
    let h = _app.window_rect().y;
    let message_screen = MessageScreen::new();
    Model {
        message_screen,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    match _event {
        Event::WindowEvent { id: _id, simple: window_event } => {
            if let Some(KeyPressed(key)) = window_event {
                match key {
                    Key::Up => _model.message_screen.add_message("hello", _app),
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

    _model.message_screen.draw(&draw, 64.0);

    draw.to_frame(_app, &_frame).unwrap();
}
