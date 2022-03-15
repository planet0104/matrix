#![windows_subsystem = "windows"]

use anyhow::{Result, Ok};
use characters::{init, CharacterString};
use config::{read_config, load_font, Config};
use minifb::{Key, Window, WindowOptions};
use raqote::{ DrawTarget, SolidSource };

mod characters;
mod config;

fn on_load(window: &Window, config:&Config) -> (DrawTarget, Vec<CharacterString>){
    let size = window.get_size();
    let dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    let strings = init(&config, &window);
    (dt, strings)
}

fn main() -> Result<()> {

    let config = read_config();
    //加载字体耗时时间比较长
    let font = load_font(&config)?;

    let background_color = SolidSource::from(config.background());

    let mut window = Window::new(
        "matrix",
        config.window_width() as usize,
        config.window_height() as usize,
        WindowOptions{
            resize: true,
            // borderless: config.fullscreen(),
            none: config.fullscreen(),
            ..Default::default()
        },
    )?;

    window.limit_update_rate(Some(std::time::Duration::from_millis(config.frame_delay())));
    
    let mut size = window.get_size();
    let (mut dt, mut strings) = on_load(&window, &config);
    
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let new_size = window.get_size();
        if new_size.0 != size.0 || new_size.1 != size.1{
            size = new_size;
            let (new_dt, new_strings) = on_load(&window, &config);
            dt = new_dt;
            strings = new_strings;
        }
        
        dt.clear(background_color);
        for st in &mut strings {
            st.update();
            st.draw(&mut dt, &font);
        }

        window
            .update_with_buffer(dt.get_data(), dt.width() as usize, dt.height() as usize)
            .unwrap();
    }

    Ok(())
}