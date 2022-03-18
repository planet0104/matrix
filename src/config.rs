use anyhow::Result;
use app_dirs::{AppDataType, AppInfo};
use font_kit::font::Font;
use once_cell::sync::Lazy;
use raqote::Color;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
    vec,
};

// 凤凰点阵体
const FONT_VONWAON: &[u8] = include_bytes!("../fonts/VonwaonBitmap-16px.ttf");
// 小篆
const FONT_XIAO_ZHUAN: &[u8] = include_bytes!("../fonts/xiaozhuan.ttf");
// 方正甲骨文
const FONT_FZ_JIAGUWEN: &[u8] = include_bytes!("../fonts/FZJiaGuWen.ttf");

pub const CHARACTERS_JAP: &str = r#"
    アイウエオカキクケコサシスセソタチツテトナニヌ
    ネハヒフヘホマミムメモヤユヨラリルレワヰヱヲン・"#;
pub const CHARACTERS_01: &str = "01";
pub const CHARACTERS_JIAGUWEN: &str = r#"
㐁㐭㓞㕚㕣㚔㚤㛸㝛㝵㠯㦰㦵㨉㪔㪿㫃㯟㯥㱃㱿㲋㳄㳑㹞㺇㻎㽙㿝䇂䊤䍜䍩䎽䖵䡴䢔䮯䲨䵼
一丁丂七万三上下不丏丐丑且丕丘丙丞並丩中丮丯丰丹主丽乂乃乇之乍乎乘乙九乞乳亅事二
于云五井亘亙亞亟亡亢亥亦亨享京亯人仄今介从令以任企伇伊伏伐休伯何余作使侃來侖侚供
侯侵係俘保俞倉倗偁備傳僤允元兄兆先光克兌免兒兔兕入兩八公六兮共兵其具典兹冉册再冎
冒冓冕冗冟冠冡冬几凡出函刀刁刃分刈刖刜初删別利别刵刺則剌剛割劓力劦助勞勹勿匄匆匕
化北匚匝匿區十千卅升午卌卒協南卜占卣卩卯印卲即卿厀厃厚原厥厷去叀叁參又及友反叔取
受叜叟叡口古叩召可史右司各合吉同名吏向吕君启吹呂告周呪呼命咎咒咸品員唐唬唯商啓啚
啟喁喜喦喪單嗅嗇嘉噎嚨嚴四囟因囧困固囿圉國圍圓土在圭坎坐城域埶執堇堯塞墉壇壬壴壺
夆夌复夏夒夕夗夙多夜夢大夨天夫央夷夸夾奏契奔奚奠奡女奻好如妃妊妌妍妝妣妥妸妹妻妾
姃姓委姛姜姝姪姫姬姸姼娕娘娠娥娩娶婡婢婤婦婭媚媟媳嫀嬂嬉子孕孚孛季孫孰學孽宀宁它
宅守安宋宓宕宗官定宛宜宣室宧宫宬宮宰害宵家宷宿寅寇寍寐寒寢寤寧審寮寶尃射將專尊尋
尌對小少尗尚尞尨就尸尹尻尾尿屎履屮屯屰山岳嶲巂川州巡巢工左巫己已巳巴巸帀市帚帛帝
帥師帶帽干年并幼幽庇床庚度庭庶康庸廄廌廩延建廼廾廿弋弓弔引弗弘弜弟弦強彔彖彗彘彝
彡彭彶往征律後徏徒得徝從御復微徵徹心必念恆恒恙息悤惠慶懋懸戈戉戊戌戍戎成我戒戔戕
或戚戠截戴户才扶承抆抑折抱抿拇拯拱振捪捷掃授掔探掫援搔摣撣擇擒攣攴攵攸改敎敏敖敗
教敝敢散敦文斗斝斤斧新方斿旁旅旋族旐旡既日旦旨旬旾昃昌明昏易昔昜星春昱昷晉晝晨晴
晵晶智暈暮暴曰曲更曹曼曾替朁會月有朊朋服朕朙望朝朢木未朮朱朿杉杕杞束東杳杵析林枚
枹枼柏柲柳栽桐桑梁棄棋棗棘森椎楚極榆槁樁樂樴樹橐檀櫅櫑櫓櫛櫟欠次欶止正此步武歲歷
歸歺死殊殞殟殲殷殺毋母每毓比氏民氒气水永求汏汜汝汫沁沈沉沓沖沙沚沝沬河泉泊注洀洗
洚洛洱洲洹派涂涉涎涵涷淮深淵温湄湡溢溫溼溽滅滳漁潢潦潾澅澫濘濞濤濩濼瀑瀕瀧瀼灂火
災灾炋炎為烈烕焚焛無焱熊熟熹燀燎燕燮爪爭爯爰爲爵父爻爽爾爿牀牆牛牝牟牡牢牧物牽犬
狐狩狼狽猱猶猷獏獲獸獻率玉王玨珏現琮瑟璞璧瓚甗甘生用甫田由甲申男甹甾畀畋畏畐畜畢
畫畯異疇疋疌疐疑疒疾痍癭癸癹登發白百皆皇皮皿盂益盍盟盡監盤盧目直相盾省眉眔眚眴眾
睫瞽矢知石砅砋硪磬示祀祈祏祐祖祗祝神祠祫祭祼禘禦禫禱禽禾秀秉秋秜秦秫稼稽穆穗穫突
窺立竝竟童競竹竽笮箕箙簋簟米粦粼糞索紳終絲網編縣缶罈网罔罙罝罩置罷羅羆羈羊羋羌美
羔羞義羲羴翌習翟翦翼老耋耑耤耳聖聞聯聰聲聶聽聾聿肆肇肈肉肘肩肱育胄背胵能腋腰腹膏
膝膺臀臣臤臧臨自臬臭至臺臽臾舂興舊舌舍舝舞舟般良艱艾艿芒芟芻苑苞若茍茨莫莽萅萈萑
萬葉葬蒙蒿蓑蔑蔖蔡蕘薛薦薪藝虍虎虐虒虘虞虣虤虫虹虺蛇蛛蜀蝠融蠃蠢蠱血衆行衍衛衣衰
袁裘襄襲西要覃見視觀角解觴言訊訢誅谷豆豊豐豕豖豚象豪豭豹貔貝貞貪責買賈賓賞赤走趾
踊踰躋身車輦轡辛辟辭辰農达迅迺追退逆逋逐通速逢進逴逸逾遘遝遠遣遭遲遼還邇邊邍邑邕
郊郭鄉鄙鄰酉酌配酒酓醜采釋重量鉞錫鍥鏑鑄鑊鑿長門閤闔阜阱防降陟陮陰陶陷陸陽隹隻雀
集雇雈雉雋雍雔雚雛雝雞雥雨雩雪雲雷雹電震霋霍霖霝霰霸霾靃非面革鞭韋頁項須頤頮頻顛
風食飲飽餗饈饗首馘香馬駁駛駜騩騽驅驑驟驪骨高髟髦髭鬥鬮鬯鬱鬲鬳鬼鬽魚魯鯀鳥鳧鳳鳴
鳶鴻鷄鹵鹿麇麋麐麓麗麟麥黃黍黑黹鼄鼉鼎鼓鼜鼠鼻齊齒齲龍龐龔龜龠龢"#;
pub const CHARACTERS_ZHUANTI: &str = r#"
一乙九了七八厂儿二几力人入十又乃丁卜刀三上下与也之于千及大干工己口山才土小子久丸
丈勺刃凡亡叉川寸弓巾女尸士夕中不公六切元五今化什反天引少比斗方火毛片气日手水王文
心月支分丰乏丹予丑勿允互井云匹凶介仇仆仁仍升午友屯夫巨尺巴幻尤孔父斤木牛欠犬氏瓦
牙止爪且世主包北加出代半去平布市叫可史只它四外本民必正白立目生石示用乎丘丙占刊兄
印功令付仔失央巧左句古司台右召宁奴犯尼扔汁圣幼冬孕末未旦永甘瓜禾矛母皮甲申田穴玉"#;

pub const APP_DATA_TYPE: AppDataType = AppDataType::UserConfig;

pub static APP_INFO: Lazy<AppInfo> = Lazy::new(|| AppInfo {
    name: "matrix".into(),
    author: "planet0104.github.io".into(),
});

pub fn get_app_dir() -> Option<PathBuf> {
    //自动创建
    let path = app_dirs::app_dir(APP_DATA_TYPE, &APP_INFO, "");
    let path = if path.is_err() {
        eprintln!("APP_DIR文件夹创建失败: {:?}", path.err());
        let mut path = dirs::config_dir().unwrap_or(std::env::temp_dir());
        path.push("planet0104.github.io");
        path.push("matrix");
        path
    } else {
        path.unwrap()
    };

    match fs::create_dir_all(&path) {
        Ok(..) => Some(path),
        Err(e) => {
            eprintln!("临时文件路径读取失败:{:?}", e);
            None
        }
    }
}

pub fn get_config_path() -> Option<String> {
    let app_dir = get_app_dir();
    if app_dir.is_none() {
        return None;
    }
    let mut dir = app_dir.unwrap();

    let dir_str = dir.to_str();
    if dir_str.is_none() || dir_str.unwrap().len() == 0 {
        None
    } else {
        dir.push("Config.toml");
        match dir.to_str() {
            Some(path) => Some(path.to_string()),
            None => None,
        }
    }
}

/// 保存配置文件
pub fn write_config(cfg: &Config) -> Result<()> {
    let cfg_str = toml::to_string(&cfg)?;
    if let Some(path) = get_config_path() {
        let mut cfg_file = File::create(&path)?;
        // 用户文件夹只能创建，不能修改
        // let mut cfg_file = File::open(&path).unwrap_or(File::create(&path)?);
        cfg_file.write_all(cfg_str.as_bytes())?;
    }
    Ok(())
}

pub fn read_config() -> Config {
    read_config_file().unwrap_or(Config::default())
}

pub fn read_config_file() -> Option<Config> {
    let mut cfg = None;

    if let Some(path) = get_config_path() {
        match File::open(path) {
            Ok(mut cfg1) => {
                let mut cfg_str = String::new();
                match cfg1.read_to_string(&mut cfg_str) {
                    Ok(..) => match toml::from_str::<Config>(&cfg_str) {
                        Ok(cfg1) => {
                            cfg = Some(cfg1);
                        }
                        Err(err) => {
                            eprintln!("配置文件解析出错:{}", err);
                        }
                    },
                    Err(err) => {
                        eprintln!("配置文件读取出错:{}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("配置文件读取出错:{}", err);
            }
        }
    }
    cfg
}

/// 字体 "1"->凤凰点阵体 "2"->小篆 "3"->甲骨文 "字体文件名.ttf"->自定义ttf文件
pub fn load_font(cfg: &Config) -> Result<Font> {
    let font_name = cfg.font.clone();

    let bytes = if font_name == "2" {
        FONT_XIAO_ZHUAN.to_vec()
    } else if font_name == "3" {
        FONT_FZ_JIAGUWEN.to_vec()
    } else if font_name != "1" && font_name.len() > 0 {
        let mut f = File::open(font_name)?;
        let mut bytes = vec![];
        f.read_to_end(&mut bytes)?;
        bytes
    } else {
        FONT_VONWAON.to_vec()
    };
    Ok(Font::from_bytes(Arc::new(bytes), 0)?)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    characters: String,
    /// 字体 "1"->凤凰点阵体 "2"->小篆 "3"->甲骨文 "字体文件名.ttf"->自定义ttf文件
    pub font: String,
    pub font_size: i32,
    pub color: String,
    pub light_color: String,
    pub light_speed: i32,
    pub background: String,
    pub fade_speed: i32,
    pub spaceing: u32,
    pub fullscreen: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub logical_size: u32,
    pub mutation_rate: f32,
    pub frame_delay: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // characters: base64::encode("01".as_bytes()),
            characters: "01".to_string(),
            font: "1".to_string(),
            font_size: 12,
            color: "rgb(0, 255, 70)".to_string(),
            light_color: "white".to_string(),
            light_speed: 200,
            background: "black".to_string(),
            fade_speed: 10,
            spaceing: 0,
            #[cfg(debug_assertions)]
            fullscreen: false,
            #[cfg(not(debug_assertions))]
            fullscreen: true,
            window_width: 900,
            window_height: 600,
            logical_size: 640,
            mutation_rate: 0.001,
            #[cfg(debug_assertions)]
            frame_delay: 2000,
            #[cfg(not(debug_assertions))]
            frame_delay: 50,
        }
    }
}

impl Config {
    pub fn parse_color(color: &str, default: csscolorparser::Color) -> Color {
        let color = csscolorparser::parse(color).unwrap_or(default);

        Color::new(
            (color.a * 255.0) as u8,
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        )
    }

    pub fn characters(&self) -> String {
        // let bytes = base64::decode(&self.characters).expect("解析失败");
        // String::from_utf8_lossy(&bytes).to_string()
        self.characters.clone()
    }

    pub fn characters_plain(&self) -> String {
        self.characters
            .trim()
            .replace("\t", "")
            .replace("\r", "")
            .replace("\n", "")
            .replace(" ", "")
    }

    pub fn set_characters(&mut self, characters: &str) {
        // let encoded = base64::encode(characters.as_bytes());
        self.characters = characters.to_string();
    }

    pub fn color(&self) -> Color {
        Self::parse_color(&self.color, csscolorparser::Color::from_rgb_u8(0, 255, 70))
    }

    pub fn light_color(&self) -> Color {
        Self::parse_color(
            &self.light_color,
            csscolorparser::Color::from_rgb_u8(255, 255, 255),
        )
    }

    pub fn background(&self) -> Color {
        Self::parse_color(
            &self.background,
            csscolorparser::Color::from_rgb_u8(0, 0, 0),
        )
    }
}
