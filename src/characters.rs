use font_kit::font::Font;
use rand::{prelude::ThreadRng, Rng};
use raqote::{Color, DrawOptions, DrawTarget, Point, Source};

use crate::config::Config;

pub struct Character {
    pub pos: Point,
    pub font_size: f32,
    pub tile: String,
    pub color: Color,
    //白光 即覆盖文字
    pub light_color: Color,
    pub light_speed: i32,
    pub fade_speed: i32,
    //透明度变为0以后，消失不可见
    pub tile_visible: bool,
    pub light_visible: bool,
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
            light_speed: 10,
            fade_speed: 10,
            tile_visible: true,
            light_visible: true,
        }
    }
}

impl Character {
    pub fn update(&mut self) {
        if self.tile_visible && self.color.a() == 0 {
            self.tile_visible = false;
        }
        if self.light_visible && self.light_color.a() == 0 {
            self.light_visible = false;
        }
        if !self.tile_visible && !self.light_visible {
            return;
        }

        //更新闪光文字透明色
        if self.light_color.a() > 0 {
            let mut alpha = self.light_color.a() as i32;
            alpha -= self.light_speed;
            if alpha < 0 {
                alpha = 0;
            }
            self.light_color = Color::new(
                alpha as u8,
                self.light_color.r(),
                self.light_color.g(),
                self.light_color.b(),
            );
        }

        // 更新文字透明色
        if self.color.a() > 0 {
            let mut alpha = self.color.a() as i32;
            alpha -= self.fade_speed;
            if alpha < 0 {
                alpha = 0;
            }
            self.color = Color::new(alpha as u8, self.color.r(), self.color.g(), self.color.b());
        }
    }
    pub fn draw(&self, canvas: &mut DrawTarget, font: &Font) -> i32 {
        let mut count = 0;
        if self.tile_visible {
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
        if self.light_visible {
            canvas.draw_text(
                font,
                self.font_size,
                &self.tile,
                self.pos,
                &Source::from(self.light_color),
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
    pub light_speed: i32,
    pub tiles: Vec<char>,
    pub frame_delay: u64,
    //随机延时
    delay: u64,
}

impl CharacterString {
    pub fn update(&mut self) {
        if self.delay != 0 {
            self.delay -= 1;
            return;
        }
        for c in &mut self.characters {
            c.update();
        }
        self.characters.retain(|c| c.color.a() > 0);
        for c in &mut self.characters {
            if self.rng.gen::<f32>() < self.mutation_rate {
                c.tile = self.tiles[self.rng.gen_range(0..self.tiles.len())].to_string();
            }
        }

        if self.current_index < self.max_len {
            //没有绘制结束，继续添加字符
            let y = self.current_index as f32 * (self.font_size + self.spacing as f32);
            let c = self.tiles[self.rng.gen_range(0..self.tiles.len())];
            self.characters.push(Character {
                pos: Point::new(self.x, y),
                tile: format!("{c}"),
                color: self.color,
                light_color: self.light_color,
                light_speed: self.light_speed,
                fade_speed: self.fade_speed,
                font_size: self.font_size,
                ..Default::default()
            });
            self.current_index += 1;
        } else {
            //已经绘制结束，检查是否所有字符都已消失
            if self.characters.len() == 0 {
                //重新开始新的一轮
                self.current_index = 0;
                //延迟1~6000毫秒, 每帧的时间是frame_delay
                let max = 6000 / self.frame_delay;
                self.delay = self.rng.gen_range(0..max);
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

    for col in 0..columns {
        strings.push(CharacterString {
            rng: rand::thread_rng(),
            font_size: cfg.font_size as f32,
            color,
            light_color: cfg.light_color(),
            light_speed: cfg.light_speed,
            mutation_rate: cfg.mutation_rate,
            tiles: cfg.characters_plain().chars().collect(),
            characters: vec![],
            frame_delay: cfg.frame_delay,
            max_len: rows as usize,
            current_index: 0,
            delay: 0,
            x: col as f32 * font_size as f32,
            fade_speed: cfg.fade_speed,
            spacing: cfg.spaceing,
        });
    }

    strings
}
