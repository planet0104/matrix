// 屏保主程序
use anyhow::{anyhow, Result};
use fast_image_resize::{Image, ResizeAlg, Resizer};
use font_kit::font::Font;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use raqote::{DrawOptions, DrawTarget, SolidSource, Source};
use softbuffer::GraphicsContext;
use std::{
    num::NonZeroU32,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Icon, Window, WindowBuilder},
};

use crate::{
    characters::{init, CharacterString},
    config::{self, load_font, read_config, read_config_file, Config},
    setting::{self, alert},
};

#[derive(Debug, Clone, Copy)]
enum MyEvent {
    Redraw,
    Reload,
}

pub fn start() -> Result<()> {
    let event_loop = EventLoop::<MyEvent>::with_user_event();

    let mut config = read_config();

    let icon = image::load_from_memory(ICON)?.to_rgba8();
    let (icon_width, icon_height) = (icon.width(), icon.height());
    let icon_buf = icon.into_raw();

    let window = {
        let mut builder = WindowBuilder::new().with_title("matrix");

        if !config.fullscreen || cfg!(debug_assertions) {
            //非全屏模式设置窗口大小
            builder = builder.with_inner_size(LogicalSize::new(
                config.window_width as f64,
                config.window_height as f64,
            ))
        } else {
            builder = builder.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        builder
            .with_window_icon(Some(Icon::from_rgba(icon_buf, icon_width, icon_height)?))
            .build(&event_loop)?
    };

    let mut graphics_context =
        unsafe { GraphicsContext::new(window) }.map_err(|err| anyhow!("{:?}", err))?;

    let monitor = graphics_context.window().current_monitor().unwrap();

    let (render_size, window_size) = if config.fullscreen {
        //全屏模式，根据渲染宽度计算渲染高度
        let screen_size = monitor.size();
        (
            aspect_size(&config, screen_size.width as f64, screen_size.height as f64),
            screen_size,
        )
    } else {
        //非全屏模式，渲染大小默认等于窗口大小, 窗口大小改变以后需要重新计算render_size
        let window_size = graphics_context.window().inner_size();
        (
            aspect_size(&config, window_size.width as f64, window_size.height as f64),
            window_size,
        )
    };

    let (mut resized_image, mut resized_dt, mut dt, mut strings) =
        on_load(render_size, window_size, config.clone()).expect("初始化失败!");

    //加载字体耗时时间比较长
    let mut font = load_font(&config)?;

    let mut background_color = SolidSource::from(config.background());

    //记录光标开始移动的时间点
    let mut last_move_time = Instant::now();
    //鼠标是否在移动
    let mut cursor_moveing = false;
    //光标开始移动时间
    let mut start_move_time = Instant::now();

    //图像缩放配置
    let mut resizer = Resizer::new(ResizeAlg::Nearest);

    // 重绘事件
    let event_loop_proxy = event_loop.create_proxy();
    let frame_delay = config.frame_delay;
    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();
    spawn(move || {
        let mut frame_delay = Duration::from_millis(frame_delay);
        loop {
            if let Ok(fd) = rx.try_recv() {
                frame_delay = Duration::from_millis(fd);
            }
            sleep(frame_delay);
            event_loop_proxy.send_event(MyEvent::Redraw).ok();
        }
    });

    //监控配置文件改动
    let event_loop_proxy = event_loop.create_proxy();
    if let Some(dir) = config::get_app_dir() {
        spawn(move || -> Result<()> {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut watcher = RecommendedWatcher::new(tx)?;

            watcher.watch(dir.as_path(), RecursiveMode::Recursive)?;

            for res in rx {
                match res {
                    Ok(_event) => {
                        event_loop_proxy.send_event(MyEvent::Reload).ok();
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
            Ok(())
        });
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if cursor_moveing {
            //超过300秒钟未移动鼠标，标记鼠标停止移动
            let now = Instant::now();
            if now - last_move_time > Duration::from_millis(300) {
                cursor_moveing = false;
            }
        }

        match event {
            Event::UserEvent(event) => match event {
                MyEvent::Redraw => {
                    if let Err(err) = redraw(
                        &mut resizer,
                        &mut resized_image,
                        &mut resized_dt,
                        &font,
                        background_color,
                        &mut dt,
                        &mut strings,
                        &mut graphics_context,
                    ) {
                        alert("错误", &format!("绘图失败:{:?}", err));
                    }
                }
                MyEvent::Reload => {
                    if let Some(cfg) = read_config_file() {
                        config = cfg;
                        if let Ok(f) = load_font(&config) {
                            font = f;
                            let _ = tx.send(config.frame_delay);
                            background_color = SolidSource::from(config.background());
                            let window_size = graphics_context.window().inner_size();
                            let render_size = aspect_size(
                                &config,
                                window_size.width as f64,
                                window_size.height as f64,
                            );
                            if let Some((ri, rd, d, s)) =
                                on_load(render_size, window_size, config.clone())
                            {
                                dt = d;
                                strings = s;
                                resized_image = ri;
                                resized_dt = rd;
                            }
                        }
                    } else {
                        println!("配置文件读取失败");
                    }
                }
            },
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                if let Err(err) = redraw(
                    &mut resizer,
                    &mut resized_image,
                    &mut resized_dt,
                    &font,
                    background_color,
                    &mut dt,
                    &mut strings,
                    &mut graphics_context,
                ) {
                    alert("错误", &format!("绘图失败:{:?}", err));
                }
            }
            Event::WindowEvent {
                event, window_id, ..
            } => {
                match event {
                    WindowEvent::Resized(..) => {
                        let window_size = graphics_context.window().inner_size();
                        let render_size = aspect_size(
                            &config,
                            window_size.width as f64,
                            window_size.height as f64,
                        );
                        if let Some((ri, rd, d, s)) =
                            on_load(render_size, window_size, config.clone())
                        {
                            dt = d;
                            strings = s;
                            resized_image = ri;
                            resized_dt = rd;
                        }
                        let w = graphics_context.window_mut();
                        w.set_decorations(!w.is_maximized());
                        // w.set_always_on_top(w.is_maximized() || w.fullscreen().is_some());
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == ElementState::Released {
                            if let Some(VirtualKeyCode::F11) = input.virtual_keycode {
                                let w = graphics_context.window_mut();
                                //如果是最大化模式，F11还原窗口
                                if w.is_maximized() {
                                    w.set_maximized(false);
                                } else {
                                    //F11全屏切换
                                    if graphics_context.window().fullscreen().is_some() {
                                        graphics_context.window().set_fullscreen(None);
                                    } else {
                                        graphics_context
                                            .window()
                                            .set_fullscreen(Some(Fullscreen::Borderless(None)));
                                    }
                                }
                            } else if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                                *control_flow = ControlFlow::Exit
                            } else if let Some(VirtualKeyCode::F1) = input.virtual_keycode {
                                setting::open_self();
                            }
                        }
                    }
                    WindowEvent::CursorMoved { position: _, .. } => {
                        last_move_time = Instant::now();
                        if !cursor_moveing {
                            cursor_moveing = true;
                            start_move_time = Instant::now();
                        } else {
                            if last_move_time - start_move_time > Duration::from_millis(400) {
                                //持续移动超过400ms，退出
                                if config.mousequit {
                                    *control_flow = ControlFlow::Exit
                                }
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        if window_id == graphics_context.window().id() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    });
}

const ICON: &[u8] = include_bytes!("../favicon.png");

/// 根据渲染大小，重新创建DrawTarget、生成CharacterString
fn on_load(
    render_size: PhysicalSize<u32>,
    window_size: PhysicalSize<u32>,
    config: Config,
) -> Option<(Image<'static>, DrawTarget, DrawTarget, Vec<CharacterString>)> {
    let dt = DrawTarget::new(render_size.width as i32, render_size.height as i32);
    let strings = init(&config, render_size.width, render_size.height);

    //缩放后的图像缓冲区
    let resized_image = Image::new(
        NonZeroU32::new(window_size.width)?,
        NonZeroU32::new(window_size.height)?,
        fast_image_resize::PixelType::U8x4,
    );
    let resized_dt = DrawTarget::new(window_size.width as i32, window_size.height as i32);
    Some((resized_image, resized_dt, dt, strings))
}
/// 根据实际窗口大小，来调整渲染大小
fn aspect_size(config: &Config, window_width: f64, window_height: f64) -> PhysicalSize<u32> {
    let mut logical_size = config.logical_size as f64;

    if logical_size > window_width {
        logical_size = window_width;
    }

    let scale = logical_size / window_width;

    let render_width = window_width * scale;
    let render_height = window_height * scale;

    PhysicalSize::new(render_width as u32, render_height as u32)
}

fn redraw(
    resizer: &mut Resizer,
    resize_image: &mut Image,
    resized_dt: &mut DrawTarget,
    font: &Font,
    background_color: SolidSource,
    dt: &mut DrawTarget,
    strings: &mut [CharacterString],
    graphics_context: &mut GraphicsContext<Window>,
) -> Result<()> {
    // dt.clear(background_color);
    if strings.len() > 0 {
        dt.fill_rect(
            0.,
            0.,
            dt.width() as f32,
            dt.height() as f32,
            &Source::Solid(SolidSource {
                r: background_color.r,
                g: background_color.g,
                b: background_color.b,
                a: (strings[0].fade_speed * 2) as u8,
            }),
            &DrawOptions::default(),
        );
    }

    let mut count = 0;
    for st in strings {
        st.update();
        count += st.draw(dt, &font);
    }
    if cfg!(debug_assertions) {
        println!("绘制了{count}个字符");
    }

    //缩放至窗口实际大小
    let window_size = graphics_context.window().inner_size();
    let image = Image::from_slice_u8(
        NonZeroU32::new(dt.width() as u32).unwrap(),
        NonZeroU32::new(dt.height() as u32).unwrap(),
        dt.get_data_u8_mut(),
        fast_image_resize::PixelType::U8x4,
    )?;
    resizer.resize(&image.view(), &mut resize_image.view_mut())?;

    resized_dt
        .get_data_u8_mut()
        .copy_from_slice(resize_image.buffer());

    graphics_context.set_buffer(
        resized_dt.get_data(),
        window_size.width as u16,
        window_size.height as u16,
    );

    Ok(())
}
