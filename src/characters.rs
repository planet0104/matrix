use std::time::{Duration, Instant};

use font_kit::font::Font;
use rand::{prelude::ThreadRng, Rng};
use raqote::{Color, DrawOptions, DrawTarget, Point, Source};

use crate::config::Config;

pub struct Character {
    pub pos: Point,
    pub font_size: f32,
    pub tile: String,
    pub color: Color,
    pub light_color: Color,
    // 0->绘制闪光 1->绘制文本
    pub step: u8,
    pub options: DrawOptions,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            pos: Point::new(0., 0.),
            font_size: 14.0,
            tile: "A".to_string(),
            color: Color::new(255, 255, 255, 255),
            light_color: Color::new(255, 255, 255, 255),
            options: DrawOptions::default(),
            step: 0,
        }
    }
}

/// 字符渲染逻辑: 绘制一次闪光色，绘制一次文字色
impl Character {
    pub fn update(&mut self) {
        if self.step < 5 {
            self.step += 1;
        }
    }

    pub fn draw(&self, canvas: &mut DrawTarget, font: &Font) -> i32 {
        let mut count = 0;
        if self.step == 0 {
            canvas.draw_text(
                font,
                self.font_size,
                &self.tile,
                self.pos,
                &Source::from(self.light_color),
                &self.options,
            );
            count += 1;
        } else if self.step == 1 {
            canvas.draw_text(
                font,
                self.font_size,
                &self.tile,
                self.pos,
                &Source::from(self.color),
                &self.options,
            );
            count += 1;
        }
        count
    }
}

pub struct CharacterString {
    rng: ThreadRng,
    //当前显示的字符
    pub characters: Vec<Character>,
    //最大字符串长度(屏幕高度/字体大小)
    pub max_len: usize,
    //当前绘制的位置
    pub current_index: usize,
    pub mutation_rate: f32,
    pub x: f32,
    pub fade_speed: i32,
    pub font_size: f32,
    pub spacing: u32,
    pub color: Color,
    pub light_color: Color,
    pub tiles: Vec<char>,
    //随机延时
    delay_time: Duration,
    start_time: Instant,
}

impl CharacterString {
    pub fn update(&mut self) {
        if self.start_time.elapsed() < self.delay_time {
            return;
        }
        for c in &mut self.characters {
            c.update();
        }
        self.characters.retain(|c| c.step < 2);

        if self.current_index < self.max_len {
            //没有绘制结束，继续添加字符
            let y = self.current_index as f32 * (self.font_size + self.spacing as f32);
            let c = self.tiles[self.rng.gen_range(0..self.tiles.len())];
            self.characters.push(Character {
                pos: Point::new(self.x, y),
                tile: format!("{c}"),
                color: self.color,
                light_color: self.light_color,
                font_size: self.font_size,
                ..Default::default()
            });
            self.current_index += 1;
        } else {
            //已经绘制结束，检查是否所有字符都已消失
            if self.characters.len() == 0 {
                //重新开始新的一轮
                self.current_index = 0;
                //延迟1~7秒
                self.start_time = Instant::now();
                self.delay_time = Duration::from_millis(self.rng.gen_range(1000..7000));
            }
        }
    }

    pub fn draw(&self, canvas: &mut DrawTarget, font: &Font) -> i32 {
        let mut count = 0;
        for c in &self.characters {
            count += c.draw(canvas, font);
        }
        count
    }
}

pub fn init(cfg: &Config, width: u32, height: u32) -> Vec<CharacterString> {
    let font_size = cfg.font_size;

    let color = cfg.color();

    let mut strings = vec![];

    let columns = width / font_size as u32;
    let rows = (height / (font_size as u32 + cfg.spaceing)) + 1;
    // println!("{width}x{height} 列数{columns}行数:{rows}");

    for col in 0..columns {
        strings.push(CharacterString {
            rng: rand::thread_rng(),
            font_size: cfg.font_size as f32,
            color,
            light_color: cfg.light_color(),
            mutation_rate: cfg.mutation_rate,
            tiles: cfg.characters_plain().chars().collect(),
            characters: vec![],
            max_len: rows as usize,
            current_index: 0,
            x: col as f32 * font_size as f32,
            fade_speed: cfg.fade_speed,
            spacing: cfg.spaceing,
            delay_time: Duration::from_secs(0),
            start_time: Instant::now(),
        });
    }

    strings
}
