use std::time::{Instant, Duration};

use font_kit::{font::Font};
use minifb::Window;
use rand::{prelude::ThreadRng, Rng};
use raqote::{DrawTarget, Point, Source, DrawOptions, Color};

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
    pub visible: bool,
    pub options: DrawOptions,
}

impl Default for Character{
    fn default() -> Self {
        Self{
            pos: Point::new(0., 0.),
            font_size: 14.0,
            tile: "A".to_string(),
            color: Color::new(255, 255, 255, 255),
            light_color: Color::new(255, 255, 255, 255),
            options: DrawOptions::default(),
            light_speed: 10,
            fade_speed: 10,
            visible: true,
        }
    }
}

impl Character {
    pub fn update(&mut self) {
        if self.visible && self.color.a() == 0{
            self.visible = false;
        }
        if !self.visible{
            return;
        }

        //更新闪光文字透明色
        if self.light_color.a() > 0{
            let mut alpha = self.light_color.a() as i32;
            alpha -= self.light_speed;
            if alpha < 0{
                alpha = 0;
            }
            self.light_color = Color::new(alpha as u8, self.light_color.r(), self.light_color.g(), self.light_color.b());
        }

        // 更新文字透明色
        if self.color.a() > 0{
            let mut alpha = self.color.a() as i32;
            alpha -= self.fade_speed;
            if alpha < 0{
                alpha = 0;
            }
            self.color = Color::new(alpha as u8, self.color.r(), self.color.g(), self.color.b());
        }
    }
    pub fn draw(&self, canvas: &mut DrawTarget, font: &Font) {
        if !self.visible {
            //减少CPU消耗
            return;
        }
        canvas.draw_text(font, self.font_size, &self.tile, self.pos, &Source::from(self.color), &self.options);
        canvas.draw_text(font, self.font_size, &self.tile, self.pos, &Source::from(self.light_color), &self.options)
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
    pub timer: AnimationTimer,
}

impl CharacterString {
    pub fn update(&mut self) {
        for c in &mut self.characters {
            c.update();
        }
        self.characters.retain(|c| c.color.a() > 0);
        for c in &mut self.characters{
            if self.rng.gen::<f32>() < self.mutation_rate{
                c.tile = self.tiles[self.rng.gen_range(0..self.tiles.len())].to_string();
            }
        }
        if self.timer.ready_for_next_frame() {
            if self.current_index < self.max_len{
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
            }else{
                //已经绘制结束，检查是否所有字符都已消失
                if self.characters.len() == 0{
                    //重新开始新的一轮
                    self.current_index = 0;
                    self.timer = AnimationTimer::new_delay(1000. / self.timer.frame_time().as_millis() as f64,
                        Duration::from_millis(self.rng.gen_range(0..6000)));
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut DrawTarget, font: &Font) {
        for c in &self.characters {
            c.draw(canvas, font);
        }
    }
}

pub fn init(cfg: &Config, window: &Window) -> Vec<CharacterString> {
    let (width, height) = window.get_size();
    let font_size = cfg.font_size() as f32;

    let color = cfg.color();

    let mut strings = vec![];

    let columns = width as u32 / font_size as u32;
    let rows =(height as u32 / (font_size as u32 + cfg.spaceing())) + 1;

    for col in 0..columns {
        strings.push(CharacterString {
            rng: rand::thread_rng(),
            font_size,
            color,
            light_color: cfg.light_color(),
            light_speed: cfg.light_speed() as i32,
            mutation_rate: cfg.mutation_rate(),
            tiles: cfg.characters().chars().collect(),
            characters: vec![],
            max_len: rows as usize,
            current_index: 0,
            x: col as f32 * font_size as f32,
            fade_speed: cfg.fade_speed() as i32,
            spacing: cfg.spaceing(),
            timer: AnimationTimer::new(1000. / cfg.step_delay() as f64),
        });
    }

    strings
}

//计时器
#[derive(Clone)]
pub struct AnimationTimer {
    frame_time: Duration,
    next_time: Instant,
}

impl AnimationTimer {
    pub fn new(fps: f64) -> AnimationTimer {
        AnimationTimer {
            frame_time: Duration::from_millis((1000.0 / fps) as u64),
            next_time: Instant::now(),
        }
    }

    //延迟一段时间开始
    pub fn new_delay(fps: f64, delay: Duration) -> AnimationTimer{
        AnimationTimer {
            frame_time: Duration::from_millis((1000.0 / fps) as u64),
            next_time: Instant::now()+delay,
        }
    }

    pub fn frame_time(&self) -> Duration{
        self.frame_time
    }

    pub fn ready_for_next_frame(&mut self) -> bool {
        if Instant::now() >= self.next_time {
            //更新时间
            self.next_time += self.frame_time;
            true
        } else {
            false
        }
    }
}
