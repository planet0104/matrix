use std::{rc::Rc, cell::RefCell};

use macroquad::{prelude::{Color, Font, load_ttf_font_from_bytes, load_file}, miniquad};
use once_cell::sync::Lazy;
use serde::Deserialize;

// 小篆
const FONT_XIAO_ZHUAN:&[u8] = include_bytes!("../fonts/xiaozhuan.ttf");
// 凤凰点阵体
const FONT_VONWAON:&[u8] = include_bytes!("../fonts/VonwaonBitmap-16px.ttf");
// 方正甲骨文
const FONT_FZ_JIAGUWEN:&[u8] = include_bytes!("../fonts/FZJiaGuWen.ttf");

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let cfg = Rc::new(RefCell::new(Some(Config::default())));
    let cfg_clone = cfg.clone();
    miniquad::fs::load_file("Config.toml", move |f|{
        if let Ok(bytes) = f{
            if let Ok(cfg_str) = String::from_utf8(bytes){
                match toml::from_str::<Config>(&cfg_str) {
                    Ok(c) => {
                        cfg_clone.borrow_mut().replace(c);
                    }
                    Err(err) => eprintln!("配置文件解析失败: {:?}", err)
                }
            }
        }
    });
    let cfg = cfg.borrow_mut().take().unwrap();
    cfg
});

pub async fn load_font() -> Font{
    let font_name = CONFIG.font.clone().unwrap_or(String::new());

    //# 字体 "1"->凤凰点阵体 "2"->小篆 "3"->甲骨文 "字体文件名.ttf"->自定义ttf文件
    if font_name == "2"{
        load_ttf_font_from_bytes(FONT_XIAO_ZHUAN).unwrap_or(Font::default())
    }else if font_name == "3"{
        load_ttf_font_from_bytes(FONT_FZ_JIAGUWEN).unwrap_or(Font::default())
    }else if font_name != "1" && font_name.len() > 0{
        match load_file(&font_name).await{
            Ok(bytes) => {
                load_ttf_font_from_bytes(&bytes).unwrap_or(Font::default())
            }
            Err(err) => {
                eprintln!("字体文件{}加载失败 {:?}", font_name, err);
                load_ttf_font_from_bytes(FONT_VONWAON).unwrap_or(Font::default())
            }
        }
    }else{
        load_ttf_font_from_bytes(FONT_VONWAON).unwrap_or(Font::default())
    }
}

#[derive(Default, Deserialize)]
pub struct Config {
    characters: Option<String>,
    font: Option<String>,
    font_size: Option<u16>,
    color: Option<String>,
    background: Option<String>,
    fade_delay: Option<u32>,
    step_delay: Option<u32>,
    spaceing: Option<u32>,
    fullscreen: Option<bool>,
    window_width: Option<u32>,
    window_height: Option<u32>,
    mutation_rate: Option<f32>,
}

impl Config {
    fn parse_color(color:Option<&String>, default: &str) -> Color{
        let color = csscolorparser::parse(color.unwrap_or(&default.to_string()))
            .unwrap_or(csscolorparser::Color::from_rgb(1., 1., 1.));
        Color::from_rgba(
            (color.r * 255.) as u8,
            (color.g * 255.) as u8,
            (color.b * 255.) as u8,
            255,
        )
    }
    pub fn color(&self) -> Color {
        Self::parse_color(self.color.as_ref(), "rgb(0, 255, 70)")
    }

    pub fn background(&self) -> Color {
        Self::parse_color(self.background.as_ref(), "#000")
    }

    pub fn characters(&self) -> String {
        self.characters.clone().unwrap_or("01".to_string())
    }

    pub fn font_size(&self) -> u16 {
        self.font_size.unwrap_or(16)
    }

    pub fn spaceing(&self) -> u32 {
        self.spaceing.unwrap_or(6)
    }

    pub fn fade_delay(&self) -> u32{
        self.fade_delay.unwrap_or(16)
    }
    pub fn step_delay(&self) -> u32{
        self.step_delay.unwrap_or(100)
    }
    pub fn fullscreen(&self) -> bool{
        self.fullscreen.unwrap_or(true)
    }

    pub fn window_width(&self) -> u32{
        self.window_width.unwrap_or(640)
    }

    pub fn window_height(&self) -> u32{
        self.window_height.unwrap_or(480)
    }

    pub fn mutation_rate(&self) -> f32{
        self.mutation_rate.unwrap_or(0.)
    }
}