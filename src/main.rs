#![windows_subsystem = "windows"]

use macroquad::{prelude::*, window, miniquad::conf::Icon};
mod characters;
mod config;
mod icon;
use icon::*;
use config::*;
use characters::*;

fn window_conf() -> Conf {
    Conf {
        icon: Some(Icon{
            small: ICON_16,
            medium: ICON_32,
            big: ICON_64,
        }),
        window_title: "Matrix".to_owned(),
        fullscreen: CONFIG.fullscreen(),
        window_width: CONFIG.window_width() as i32,
        window_height: CONFIG.window_height() as i32,
        high_dpi: false,
        sample_count: 1,
        window_resizable: true,
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    //加载字体耗时时间比较长
    let font = load_font().await;

    let background_color = CONFIG.background();
    let mut strings = init(font);

    let mut width = window::screen_width();
    let mut height = window::screen_height();

    loop {

        let (w, h) = ( window::screen_width(), window::screen_height() );

        if w != width || h != height{
            strings = init(font);
            width = w;
            height = h;
        }

        clear_background(background_color);

        for st in &mut strings {
            st.update();
            st.draw();
        }

        next_frame().await
    }
}