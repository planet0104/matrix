// 屏保主程序
use anyhow::{anyhow, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    thread::spawn,
    time::{Duration, Instant},
    sync::{Arc, Mutex}
};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings, Filter, GlyphCache};
use piston::{event_loop::{EventSettings, Events}, ButtonEvent, ButtonState, Button, Key, ResizeEvent, MouseCursorEvent};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const OPEN_GL_VERSION: OpenGL = OpenGL::V2_1;

use crate::{
    characters::{init, CharacterString},
    config::{self, read_config, Config, load_font_file, FONT_VONWAON, write_config},
    setting,
};

pub struct App<'a> {
    gl: GlGraphics,
    glyphs: GlyphCache<'a>,
    strings: Vec<CharacterString>,
    config: Config,
    background: [f32; 4],
    loading: bool,
}

impl <'a> App<'a> {
    pub fn new() -> Result<Self>{
        Ok(Self{
            gl: GlGraphics::new(OPEN_GL_VERSION),
            loading: true,
            glyphs: App::load_glyphs(FONT_VONWAON.to_vec())?,
            strings: vec![],
            config: Config::default(),
            background: [0.0, 0.0, 0.0, 1.0]
        })
    }

    fn load_glyphs(font: Vec<u8>) -> Result<GlyphCache<'a>>{
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let font = rusttype::Font::try_from_vec(font);
        if font.is_none(){
            return Err(anyhow!("字体文件解析失败"));
        }
        let font = font.expect("字体文件解析失败");
        Ok(GlyphCache::from_font(font, (), texture_settings))
    }

    fn reload(&mut self, config:Config, window_size: &[f64; 2]) -> Result<()>{
        self.glyphs = App::load_glyphs(load_font_file(&config))?;
        self.strings = init(&config, window_size[0] as u32, window_size[1] as u32);
        self.background = config.background();
        self.config = config;
        self.loading = false;
        Ok(())
    }

    fn render(&mut self, args: &RenderArgs){
        use graphics::*;
        let viewport = args.viewport();
        self.gl.draw(viewport, |c, gl| {

            clear(self.background, gl);
            // let mut count_str = String::new();
            for st in &self.strings {
                let mut _count = 0;
                match st.draw(&c, gl, &mut self.glyphs){
                    Err(err) => eprintln!("{:?}", err),
                    Ok(c) => _count += c
                };
                // count_str.push_str(&format!("{count},"));
            }
            // println!("共{}列", self.strings.len());
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        for st in &mut self.strings {
            st.update();
        }
    }
}

pub fn run() -> Result<()>{

    let create_window = ||{
        let config = read_config();

        let window: GlutinWindow = WindowSettings::new("matrix", [900, 600])
            .graphics_api(OPEN_GL_VERSION)
            .exit_on_esc(true)
            .fullscreen(config.fullscreen)
            .build().unwrap();
        
        let max_fps = (1000.0 / config.frame_delay as f64) as u64;

        let app = App::new().unwrap();

        let events = Events::new(EventSettings{
            max_fps,
            ups: max_fps,
            ..Default::default()
        });

        (config, app, window, events)
    };

    let mut reload = true;
    let config_change = Arc::new(Mutex::new(false));

    //监控配置文件改动
    let inner_config_change = config_change.clone();
    if let Some(dir) = config::get_app_dir() {
        spawn(move || -> Result<()> {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut watcher = RecommendedWatcher::new(tx)?;

            watcher.watch(dir.as_path(), RecursiveMode::Recursive)?;

            for res in rx {
                match res {
                    Ok(_event) => {
                        if let Ok(mut change) = inner_config_change.lock(){
                            *change = true;
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
            Ok(())
        });
    }

    let mut old_window_size:Option<[f64; 2]> = None;

    //记录光标开始移动的时间点
    let mut last_move_time = Instant::now();
    //鼠标是否在移动
    let mut cursor_moveing = false;
    //光标开始移动时间
    let mut start_move_time = Instant::now();

    while reload{
        reload = false;
        let (mut config, mut app, mut window, mut events) = create_window();

        while let Some(e) = events.next(&mut window) {

            if cursor_moveing {
                //超过300秒钟未移动鼠标，标记鼠标停止移动
                let now = Instant::now();
                if now - last_move_time > Duration::from_millis(300) {
                    cursor_moveing = false;
                }
            }

            if let Some(_args) = e.mouse_cursor_args(){
                last_move_time = Instant::now();
                if !cursor_moveing {
                    cursor_moveing = true;
                    start_move_time = Instant::now();
                } else {
                    if last_move_time - start_move_time > Duration::from_millis(600) {
                        //持续移动超过600ms，退出
                        break;
                    }
                }
            }

            if let Some(args) = e.resize_args(){
                if let Some(old_size) = old_window_size.as_ref(){
                    let dx = args.window_size[0] - old_size[0];
                    let dy = args.window_size[1] - old_size[1];
                    if dx.abs() > 0. || dy.abs() > 0.{
                        let _ = app.reload(config.clone(), &args.window_size);
                    }
                }
                old_window_size = Some(args.window_size);
            }

            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Release {
                    match k.button {
                        Button::Keyboard(Key::F11) => {
                            reload = true;
                            config.fullscreen = !config.fullscreen;
                            let _ = write_config(&config);
                            break;
                        }
                        Button::Keyboard(Key::F1) => {
                            setting::open_self();
                        }
                        _ => (),
                    }
                }
            }
            
            if let Some(args) = e.render_args() {
                if app.loading{
                    old_window_size = Some(args.window_size);
                    let _ = app.reload(config.clone(), &args.window_size);
                }
                if let Ok(mut change) = config_change.lock(){
                    if *change{
                        *change = false;
                        reload = true;
                        break;
                        // config = read_config();
                        // let _ = app.reload(config.clone(), &args.window_size);
                    }
                }
                app.render(&args);
            }

            if let Some(args) = e.update_args() {
                app.update(&args);
            }
        }
    }
    Ok(())
}

// const ICON: &[u8] = include_bytes!("../favicon.png");

// /// 根据渲染大小，重新创建DrawTarget、生成CharacterString
// fn on_load(
//     render_size: PhysicalSize<u32>,
//     window_size: PhysicalSize<u32>,
//     config: Config,
// ) -> Option<(Image<'static>, DrawTarget, DrawTarget, Vec<CharacterString>)> {
//     let dt = DrawTarget::new(render_size.width as i32, render_size.height as i32);
//     let strings = init(&config, render_size.width, render_size.height);

//     //缩放后的图像缓冲区
//     let resized_image = Image::new(
//         NonZeroU32::new(window_size.width)?,
//         NonZeroU32::new(window_size.height)?,
//         fast_image_resize::PixelType::U8x4,
//     );
//     let resized_dt = DrawTarget::new(window_size.width as i32, window_size.height as i32);
//     Some((resized_image, resized_dt, dt, strings))
// }
// /// 根据实际窗口大小，来调整渲染大小
// fn aspect_size(config: &Config, window_width: f64, window_height: f64) -> PhysicalSize<u32> {
//     let mut logical_size = config.logical_size as f64;

//     if logical_size > window_width {
//         logical_size = window_width;
//     }

//     let scale = logical_size / window_width;

//     let render_width = window_width * scale;
//     let render_height = window_height * scale;

//     PhysicalSize::new(render_width as u32, render_height as u32)
// }