#![windows_subsystem = "windows"]

use anyhow::Result;
mod app;
mod characters;
mod config;
mod setting;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    use std::env;

    use crate::setting::alert;

    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        let arg = arg.to_lowercase();
        if arg.starts_with("/p") {
            //收到 /p 参数, 结束
            return Ok(());
        }
        if arg.starts_with("/c") {
            //打开设置页面
            setting::open();
            return Ok(());
        }
    }

    //启动屏保
    if let Err(err) = app::start() {
        eprintln!("启动失败:{:?}", err);
        alert("错误", &format!("启动失败:{:?}", err));
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<()> {
    Ok(())
}
