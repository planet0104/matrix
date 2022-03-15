# matrix

[![](https://img.shields.io/badge/latest-1.0.0-red.svg)](https://github.com/planet0104/matrix/releases)

**运行仅占用1%的CPU，10M左右内存，不占用GPU资源。**

```text
运行之前，将配置文件改为fullscreen=true，然后计算并修改窗口大小
计算方式:
屏幕实际分辨率=2880x1800 缩放=200% 那么窗口宽=2880/2=1440, 高=1800/2=900
屏幕实际分辨率=1920x1080 缩放=100% 那么窗口宽=1920, 高=1080
```

## 配置文件 Config.toml
```toml
#随机字符
characters = "0123456789"

# 字体 "1"->凤凰点阵体 "2"->小篆 "3"->甲骨文 "字体文件名.ttf"->自定义ttf文件
font = "1"

# font = "fonts/VonwaonBitmap-16px.ttf"
# 字号
font_size = 16

# 字符上下间距
spaceing = 0

# 字符突变率 0不突变
mutation_rate = 0.001

# 闪光颜色
light_color="#fff"
# 闪光消失速度（alpha透明度255递减)
light_speed=200

# 消失速度 （alpha透明度255递减)
fade_speed=10

# 雨帘前进延时(毫秒)
step_delay=60

# 文字颜色
color="rgb(0, 255, 70)"

# 背景色
background="#000"

# 全屏模式
fullscreen=true

# 窗口大小
window_width=1920
window_height=1080

# 帧延时 1000/50=20帧 延时越小，CPU占用率越高
frame_delay = 50
```

## 运行截图

<img src="images/01.png" />

<img src="images/02.png" />

<img src="images/03.png" />

<img src="images/04.png" />