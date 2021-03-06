//设置程序

use std::{cell::RefCell, env::current_exe, process::Command, rc::Rc};

use crate::config::{
    read_config, write_config, CHARACTERS_01, CHARACTERS_JAP, CHARACTERS_JIAGUWEN,
    CHARACTERS_ZHUANTI,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use slint::{quit_event_loop, SharedString};

slint::slint! {
    import { SettingWindow } from "ui/setting.slint";
}

pub fn open() {
    let config = Rc::new(RefCell::new(read_config()));
    let window = SettingWindow::new();

    let config_update = config.clone();
    let update_window = window.as_weak();
    let update_values = move || {
        let cfg = config_update.borrow();
        if let Some(window) = update_window.upgrade() {
            window.set_characters(SharedString::from(cfg.characters()));
            window.set_text_color(SharedString::from(&cfg.color));
            let font_type = if cfg.font == "2" {
                "小篆"
            } else if cfg.font == "3" {
                "甲骨文"
            } else if cfg.font == "4" {
                "永无BUG"
            } else if cfg.font == "1" {
                "默认"
            } else {
                "字体文件"
            };
            window.set_font_type(SharedString::from(font_type));
            window.set_font_size(SharedString::from(&format!("{}", cfg.font_size)));
            window.set_spaceing(SharedString::from(&format!("{}", cfg.spaceing)));
            window.set_background_color(SharedString::from(&cfg.background));
            window.set_fullscreen(SharedString::from(if cfg.fullscreen {
                "是"
            } else {
                "否"
            }));
            window.set_logical_size(SharedString::from(&format!("{}", cfg.logical_size)));
            window.set_light_color(SharedString::from(&cfg.light_color));
            window.set_frame_delay(SharedString::from(&format!("{}毫秒", cfg.frame_delay)));
            window.set_fade_speed(SharedString::from(&format!("{}", cfg.fade_speed)));
            window.set_mousequit(SharedString::from(if cfg.mousequit {
                "滑动退出"
            } else {
                "不监听"
            }));
            window.set_mutation_rate(SharedString::from(&format!("{}", cfg.mutation_rate)));
        }
    };

    update_values();

    let config_change = config.clone();
    window.on_value_change(move |cmd, val| {
        let config_change = config_change.clone();
        if (move || -> bool {
            let mut cfg = config_change.borrow_mut();

            let cmd = cmd.to_string();
            let val = val.to_string();

            if cmd == "characters" {
                if val.len() > 0 {
                    cfg.set_characters(&val);
                }
                false
            } else if cmd == "characters_select" {
                if val == "小篆" {
                    cfg.set_characters(CHARACTERS_ZHUANTI);
                    cfg.font = "2".to_string();
                } else if val == "甲骨文" {
                    cfg.set_characters(CHARACTERS_JIAGUWEN);
                    cfg.font = "3".to_string();
                } else if val == "日文" {
                    cfg.set_characters(CHARACTERS_JAP);
                    cfg.font = "1".to_string();
                } else {
                    cfg.set_characters(CHARACTERS_01);
                    cfg.font = "1".to_string();
                }
                true
            } else if cmd == "font" {
                if val == "小篆" {
                    cfg.font = "2".to_string();
                } else if val == "甲骨文" {
                    cfg.font = "3".to_string();
                } else if val == "永无BUG" {
                    cfg.font = "4".to_string();
                } else if val == "字体文件" {
                    match FileDialog::new()
                        .add_filter("TTF字体文件", &["ttf"])
                        .show_open_single_file()
                    {
                        Ok(path) => {
                            if path
                                .and_then(|a| {
                                    a.to_str().and_then(|str| {
                                        cfg.font = str.to_string();
                                        Some(str.to_string())
                                    })
                                })
                                .is_none()
                            {
                                alert("提示", "未选择字体")
                            }
                        }
                        Err(err) => {
                            alert("错误", &format!("{:?}", err));
                        }
                    }
                } else {
                    cfg.font = "1".to_string();
                }
                false
            } else if cmd == "font_size" {
                cfg.font_size = val.parse().unwrap_or(12);
                false
            } else if cmd == "mousequit" {
                cfg.mousequit = val == "滑动退出";
                false
            } else if cmd == "fullscreen" {
                cfg.fullscreen = val == "是";
                false
            } else if cmd == "spaceing" {
                cfg.spaceing = val.parse().unwrap_or(0);
                false
            } else if cmd == "logical_size" {
                cfg.logical_size = val.parse().unwrap_or(640);
                false
            } else if cmd == "fade_speed" {
                cfg.fade_speed = val.parse().unwrap_or(10);
                false
            } else if cmd == "mutation_rate" {
                cfg.mutation_rate = val.parse().unwrap_or(0.001);
                false
            } else if cmd == "frame_delay" {
                cfg.frame_delay = val.replace("毫秒", "").parse().unwrap_or(50);
                false
            } else if cmd == "color" {
                if let Ok(_color) = csscolorparser::parse(&val) {
                    cfg.color = val;
                } else {
                    cfg.color = "rgb(0, 255, 70)".to_string();
                }
                false
            } else if cmd == "light_color" {
                if let Ok(_color) = csscolorparser::parse(&val) {
                    cfg.light_color = val;
                } else {
                    cfg.light_color = "white".to_string();
                }
                false
            } else if cmd == "background" {
                if let Ok(_color) = csscolorparser::parse(&val) {
                    cfg.background = val;
                } else {
                    cfg.background = "black".to_string();
                }
                false
            } else if cmd == "save" {
                //保存配置
                if let Err(err) = write_config(&cfg.clone()) {
                    eprintln!("配置文件写入失败:{:?}", err);
                }
                quit_event_loop();
                false
            } else {
                false
            }
        })() {
            update_values();
        }
    });

    window.run();
}

pub fn open_self() {
    if let Ok(exe_pah) = current_exe() {
        let _ = Command::new(exe_pah).arg("/c").spawn();
    }
}

pub fn alert(title: &str, text: &str) {
    if let Err(err) = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(text)
        .show_alert()
    {
        eprintln!("{:?}", err);
    }
}
