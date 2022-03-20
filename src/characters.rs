use std::time::{Duration, Instant};

use opengl_graphics::{GlyphCache, GlGraphics};
use rand::{prelude::ThreadRng, Rng};
use graphics::*;
use crate::config::Config;
use anyhow::{anyhow, Result};

pub struct Character {
    pub pos: [f64; 2],
    pub font_size: f64,
    pub tile: String,
    pub color: [f32; 4],
    //白光 即覆盖文字
    pub light_color: [f32; 4],
    pub light_speed: i32,
    pub fade_speed: i32,
    //透明度变为0以后，消失不可见
    pub tile_visible: bool,
    pub light_visible: bool,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            pos: [0., 0.],
            font_size: 14.0,
            tile: "A".to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            light_color: [1.0, 1.0, 1.0, 1.0],
            light_speed: 10,
            fade_speed: 10,
            tile_visible: true,
            light_visible: true,
        }
    }
}

impl Character {
    pub fn update(&mut self) {
        if self.tile_visible && self.color[3] == 0. {
            self.tile_visible = false;
        }
        if self.light_visible && self.light_color[3] == 0. {
            self.light_visible = false;
        }
        if !self.tile_visible && !self.light_visible {
            return;
        }

        //更新闪光文字透明色
        if self.light_color[3] > 0. {
            self.light_color[3] -= self.light_speed as f32 / 255.0;
            if self.light_color[3] < 0. {
                self.light_color[3] = 0.;
            }
        }

        // 更新文字透明色
        if self.color[3] > 0. {
            self.color[3] -= self.fade_speed as f32 / 255.0;
            if self.color[3] < 0. {
                self.color[3] = 0.;
            }
        }
    }

    pub fn draw(&self, c: &Context, gl:&mut GlGraphics, glyphs: &mut GlyphCache) -> Result<i32>{
        let mut count = 0;
        if self.tile_visible {
            let transform = c.transform.trans(self.pos[0], self.pos[1]);
            text(self.color, self.font_size as u32, &self.tile, glyphs, transform, gl)
            .map_err(|err| anyhow!("{}", err))?;
            count += 1;
        }

        if self.light_visible {
            let transform = c.transform.trans(self.pos[0], self.pos[1]);
            text(self.light_color, self.font_size as u32, &self.tile, glyphs, transform, gl)
            .map_err(|err| anyhow!("{}", err))?;
            count += 1;
        }
        Ok(count)
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
    pub x: f64,
    pub fade_speed: i32,
    pub font_size: f64,
    pub vspacing: f64,
    pub color: [f32; 4],
    pub light_color: [f32; 4],
    pub light_speed: i32,
    pub tiles: Vec<char>,
    pub frame_delay: u64,
    //随机延时
    start_time: Instant,
    delay_time: Duration,
}

impl CharacterString {
    pub fn update(&mut self) {
        if self.start_time.elapsed() < self.delay_time{
            return;
        }
        for c in &mut self.characters {
            c.update();
        }
        self.characters.retain(|c| c.color[3] > 0.);
        for c in &mut self.characters {
            if self.rng.gen::<f32>() < self.mutation_rate {
                c.tile = self.tiles[self.rng.gen_range(0..self.tiles.len())].to_string();
            }
        }

        if self.current_index < self.max_len {
            //没有绘制结束，继续添加字符
            let y = self.current_index as f64 * (self.font_size + self.vspacing);
            let c = self.tiles[self.rng.gen_range(0..self.tiles.len())];
            self.characters.push(Character {
                pos: [self.x, y],
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
                //延迟1~6秒
                self.start_time = Instant::now();
                self.delay_time = Duration::from_millis(self.rng.gen_range(0..6000));
            }
        }
    }

    pub fn draw(&self, context: &Context, gl:&mut GlGraphics, glyphs: &mut GlyphCache) -> Result<i32> {
        let mut count = 0;
        for c in &self.characters {
            count += c.draw(context, gl, glyphs)?;
        }
        Ok(count)
    }
}

pub fn init(cfg: &Config, width: u32, height: u32) -> Vec<CharacterString> {
    // let font_size = cfg.font_size;

    let mut strings = vec![];

    let columns = width / (cfg.font_size as u32 + cfg.hspaceing) + 1;
    let rows = (height / (cfg.font_size as u32 + cfg.vspaceing)) + 2;

    // println!("init {width}x{height} 列数{columns} 行数{rows} font_size={} split_size={split_size}", cfg.font_size);

    for col in 0..columns {
        strings.push(CharacterString {
            rng: rand::thread_rng(),
            font_size: cfg.font_size as f64,
            color: cfg.color(),
            light_color: cfg.light_color(),
            light_speed: cfg.light_speed,
            mutation_rate: cfg.mutation_rate,
            tiles: cfg.characters_plain().chars().collect(),
            characters: vec![],
            frame_delay: cfg.frame_delay,
            max_len: rows as usize,
            current_index: 0,
            delay_time: Duration::from_secs(0),
            start_time: Instant::now(),
            x: col as f64 * (cfg.font_size as u32 + cfg.hspaceing) as f64,
            fade_speed: cfg.fade_speed,
            vspacing: cfg.vspaceing as f64,
        });
    }

    strings
}
