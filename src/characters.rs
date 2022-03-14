use macroquad::{prelude::{Color, Font, draw_text_ex, TextParams}, window};
use quad_rand as qrand;
use crate::config::CONFIG;

pub struct Character {
    pub x: f32,
    pub y: f32,
    pub tile: String,
    pub color: Color,
    pub timer: AnimationTimer,
}

impl Character {
    pub fn update(&mut self) {
        if self.timer.ready_for_next_frame() {
            let mut alpha = self.color.a - 0.005;
            if alpha < 0.0{
                alpha = 0.0;
            }
            self.color.a = alpha;
        }
    }
    pub fn draw(&self, font: Font, font_size: u16) {
        draw_text_ex(
            &self.tile,
            self.x,
            self.y,
            TextParams {
                color: self.color,
                font,
                font_size,
                ..Default::default()
            },
        );
    }
}

pub struct CharacterString {
    //当前显示的字符
    pub characters: Vec<Character>,
    //最大字符串长度(屏幕高度/字体大小)
    pub max_len: usize,
    //当前绘制的位置
    pub current_index: usize,
    pub mutation_rate: f32,
    pub x: f32,
    pub fade_delay: f64,
    pub font_size: f32,
    pub spacing: u32,
    pub font: Font,
    pub color: Color,
    pub tiles: Vec<char>,
    pub timer: AnimationTimer,
}

impl CharacterString {
    pub fn update(&mut self) {
        for c in &mut self.characters {
            c.update();
        }
        self.characters.retain(|c| c.color.a > 0.);
        for c in &mut self.characters{
            if qrand::gen_range(0., 1.0) < self.mutation_rate{
                c.tile = self.tiles[qrand::gen_range(0, self.tiles.len())].to_string();
            }
        }
        if self.timer.ready_for_next_frame() {
            if self.current_index < self.max_len{
                //没有绘制结束，继续添加字符
                let y = self.current_index as f32 * (self.font_size + self.spacing as f32);
                let c = self.tiles[qrand::gen_range(0, self.tiles.len())];
                self.characters.push(Character {
                    x: self.x,
                    y,
                    tile: format!("{c}"),
                    color: self.color,
                    timer: AnimationTimer::new(1000.0 / self.fade_delay),
                });
                self.current_index += 1;
            }else{
                //已经绘制结束，检查是否所有字符都已消失
                if self.characters.len() == 0{
                    //重新开始新的一轮
                    self.current_index = 0;
                    self.timer = AnimationTimer::new_delay(1000. / self.timer.frame_time(),
                        qrand::gen_range(0., 10.) * 1000.);
                }
            }
        }
    }

    pub fn draw(&self) {
        for c in &self.characters {
            c.draw(self.font, self.font_size as u16);
        }
    }
}

pub fn init(font: Font) -> Vec<CharacterString> {
    qrand::srand(instant::now() as u64);
    let width = window::screen_width();
    let height = window::screen_height();
    let font_size = CONFIG.font_size() as f32;

    let color = CONFIG.color();

    let mut strings = vec![];

    let columns = width as u32 / font_size as u32;
    let rows =(height as u32 / (font_size as u32 + CONFIG.spaceing())) + 1;

    for col in 0..columns {
        strings.push(CharacterString {
            font_size,
            font,
            color,
            mutation_rate: CONFIG.mutation_rate(),
            tiles: CONFIG.characters().chars().collect(),
            characters: vec![],
            max_len: rows as usize,
            current_index: 0,
            x: col as f32 * font_size as f32,
            fade_delay: CONFIG.fade_delay() as f64,
            spacing: CONFIG.spaceing(),
            timer: AnimationTimer::new(1000. / CONFIG.step_delay() as f64),
        });
    }

    strings
}

//计时器
#[derive(Clone)]
pub struct AnimationTimer {
    frame_time: f64,
    next_time: f64,
}

impl AnimationTimer {
    pub fn new(fps: f64) -> AnimationTimer {
        AnimationTimer {
            frame_time: 1000.0 / fps,
            next_time: instant::now(),
        }
    }

    //延迟一段时间开始
    pub fn new_delay(fps: f64, delay: f64) -> AnimationTimer{
        AnimationTimer {
            frame_time: 1000.0 / fps,
            next_time: instant::now()+delay,
        }
    }

    pub fn frame_time(&self) -> f64{
        self.frame_time
    }

    pub fn ready_for_next_frame(&mut self) -> bool {
        let now = instant::now();
        if now >= self.next_time {
            //更新时间
            self.next_time += self.frame_time;
            true
        } else {
            false
        }
    }
}
