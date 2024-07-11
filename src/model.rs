use nannou::prelude::*;
use nannou::text::{Font, FontSize, Justify};

pub struct Message {
    p: Point2,
    body: String,
}

impl Message {
    pub fn new(p: Point2, body: &str) -> Self {
        Message { p, body: body.to_string() }
    }
}

pub struct MessageScreen {
    messages: Vec<Message>,
    font: Font
}

impl MessageScreen {
    pub fn new(font: Font) -> Self {
        MessageScreen {
            messages: Vec::new(),
            font,
        }
    }

    pub fn add_message(&mut self, body: &str, app: &App) {
        let v = app.window_rect().y;
        let h = app.window_rect().x;
        let y = random_range(v.start, v.end);
        let p = pt2(h.end + 400.0, y);
        self.messages.push(Message::new(p, body));
    }

    pub fn update(&mut self, app: &App) {
        let h = app.window_rect().x;
        for message in &mut self.messages {
            message.p.x -= (h.end - h.start) / 400.0;
        }
        self.messages.retain(|message| message.p.x > h.start - 2500.0);
    }

    pub fn draw(&self, draw: &Draw, size: f32) {
        self.messages.iter().for_each(|message| {
            draw.text(&message.body)
                .font(self.font.clone())
                .xy(message.p)
                .font_size(FontSize::from_f32(size).unwrap())
                .justify(Justify::Left)
                .no_line_wrap()
                .color(WHITE);
        });
    }
}
