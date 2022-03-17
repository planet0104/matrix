# matrix 屏保

[![](https://img.shields.io/badge/latest-1.0.2-red.svg)](https://github.com/planet0104/matrix/releases)

**运行仅占用1~3%的CPU，20M左右内存，不占用GPU资源。**

## 安装屏保

右键点击matrix.scr，选择“安装”

<img src="images/install.png" />

打开屏幕保护程序，选择matrix

<img src="images/setscr.png" />

## 快捷键

### 进入全屏/退出全屏

*F11*

### 退出程序
*ESC、Alt+F4、移动鼠标*

## 配置文件 Config.toml

*在程序文件所在目录下创建Config.toml，所有配置均为可选项*

```toml
# 随机字符
characters = "01"

# 字体配置 "1"->凤凰点阵体 "2"->小篆 "3"->甲骨文 "字体文件名.ttf"->自定义ttf文件
font = "1"

# 字号
font_size = 12

# 字符间距
spaceing = 0

# 字符突变率 0不突变
mutation_rate = 0.001

# 闪光颜色
light_color="#fff"

# 闪光消失速度
light_speed=200

# 消失速度
fade_speed=10

# 前进延时
frame_delay = 50

# 文字颜色
color="rgb(0, 255, 70)"

# 背景色
background="#000"

# 全屏模式
fullscreen=true

# 逻辑渲染分辨率，值越大显示效果越清晰，但是CPU占用率也越高（最高不超过窗口物理分辨率）
logical_size=640

# 窗口大小(全屏模式不起作用)
window_width=900
window_height=600
```

## 运行截图

<img src="images/01.png" />

<img src="images/02.png" />

<img src="images/03.png" />

<img src="images/04.png" />
