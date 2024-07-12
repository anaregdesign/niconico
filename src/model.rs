use std::ops::Range;

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
        let v_padded = v.pad(50.0);
        let h = app.window_rect().x;
        let height = v_padded.end - v_padded.start;
        // split h to 5 ranges
        let num_part = 5;
        let slots: Vec<Range<f32>> = (0..num_part).map(|i| {
            (v_padded.start + height / num_part as f32 * i as f32)..(v_padded.start + height / num_part as f32 * (i + 1) as f32)
        }).collect();
        println!("slots{:?}", slots);
        let mut counts: Vec<i32> = (0..num_part).map(|i| { 0 }).collect();

        for m in &self.messages {
            for i in 0..num_part {
                if slots[i].start < m.p.y && m.p.y <= slots[i].end {
                    counts[i] += 1
                }
            }
        }
        println!("counts{:?}", counts);

        let min_count = counts.iter().min().unwrap();
         let min_slot_index = counts.iter().position(|e| e == min_count).unwrap();
        let y = random_range(slots[min_slot_index].start, slots[min_slot_index].end);

        let p = pt2(h.end + 400.0, y);
        self.messages.push(Message::new(p, body));
    }

    pub fn update(&mut self, app: &App) {
        let h = app.window_rect().x;
        for message in &mut self.messages {
            message.p.x -= 3.0 + (message.body.len() as f32).sqrt();
        }
        self.messages.retain(|message| message.p.x > h.start - 1200.0);
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
