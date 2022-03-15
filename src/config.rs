use std::{fs::File, io::Read, sync::Arc, vec};
use font_kit::font::Font;
use raqote::Color;
use serde::Deserialize;
use anyhow::Result;

// 凤凰点阵体
const FONT_VONWAON:&[u8] = include_bytes!("../fonts/VonwaonBitmap-16px.ttf");
// 小篆
const FONT_XIAO_ZHUAN:&[u8] = include_bytes!("../fonts/xiaozhuan.ttf");
// 方正甲骨文
const FONT_FZ_JIAGUWEN:&[u8] = include_bytes!("../fonts/FZJiaGuWen.ttf");

pub fn read_config() -> Config{
    let mut cfg = Config::default();

    if let Ok(mut cfg1) = File::open("Config.toml"){
        let mut cfg_str = String::new();
        if let Ok(_) = cfg1.read_to_string(&mut cfg_str){
            if let Ok(cfg1) = toml::from_str(&cfg_str){
                cfg = cfg1;
            }
        }
    }
    cfg
}

/// 字体 "1"->凤凰点阵体 "2"->小篆 "3"->甲骨文 "字体文件名.ttf"->自定义ttf文件
pub fn load_font(cfg: &Config) -> Result<Font>{

    let font_name = cfg.font.clone().unwrap_or("".to_string());

    let bytes = if font_name == "2"{
        FONT_XIAO_ZHUAN.to_vec()
    }else if font_name == "3"{
        FONT_FZ_JIAGUWEN.to_vec()
    }else if font_name != "1" && font_name.len() > 0{
        let mut f = File::open(font_name)?;
        let mut bytes = vec![];
        f.read_to_end(&mut bytes)?;
        bytes
    }else{
        FONT_VONWAON.to_vec()
    };
    Ok(Font::from_bytes(Arc::new(bytes), 0)?)
}

#[derive(Default, Deserialize)]
pub struct Config {
    characters: Option<String>,
    font: Option<String>,
    font_size: Option<u16>,
    color: Option<String>,
    light_color: Option<String>,
    light_speed: Option<u32>,
    background: Option<String>,
    fade_speed: Option<u32>,
    step_delay: Option<u32>,
    spaceing: Option<u32>,
    fullscreen: Option<bool>,
    window_width: Option<u32>,
    window_height: Option<u32>,
    mutation_rate: Option<f32>,
    frame_delay: Option<u64>,
}

impl Config {
    fn parse_color(color:Option<&String>, default: &str) -> Color{
        let color = csscolorparser::parse(color.unwrap_or(&default.to_string()))
            .unwrap_or(csscolorparser::Color::from_rgb(1., 1., 1.));

            Color::new((color.a * 255.0) as u8, (color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8)
    }
    pub fn color(&self) -> Color {
        Self::parse_color(self.color.as_ref(), "rgb(0, 255, 70)")
    }

    pub fn light_color(&self) -> Color {
        Self::parse_color(self.light_color.as_ref(), "#fff")
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
        self.spaceing.unwrap_or(0)
    }

    pub fn fade_speed(&self) -> u32{
        self.fade_speed.unwrap_or(10)
    }
    pub fn step_delay(&self) -> u32{
        self.step_delay.unwrap_or(60)
    }
    pub fn light_speed(&self) -> u32{
        self.light_speed.unwrap_or(200)
    }
    pub fn fullscreen(&self) -> bool{
        self.fullscreen.unwrap_or(false)
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
    pub fn frame_delay(&self) -> u64{
        self.frame_delay.unwrap_or(50)
    }
}