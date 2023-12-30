// colors.rs
#[derive(Eq, PartialEq, Hash)]
pub enum Colors {
    Normal,
    Aqua,
    Aquamarine1,
    Aquamarine1a,
    Aquamarine3,
    Black,
    Blue1,
    Blue3,
    Blue3a,
    BlueViolet,
    Blue,
    CadetBlue,
    CadetBluea,
    Chartreuse1,
    Chartreuse2,
    Chartreuse2a,
    Chartreuse3,
    Chartreuse3a,
    Chartreuse4,
    CornflowerBlue,
    Cornsilk1,
    Cyan1,
    Cyan2,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkMagentaa,
    DarkOliveGreen1,
    DarkOliveGreen1a,
    DarkOliveGreen2,
    DarkOliveGreen3,
    DarkOliveGreen3a,
    DarkOliveGreen3b,
    DarkOrange3,
    DarkOrange3a,
    DarkOrange,
    DarkRed,
    DarkReda,
    DarkSeaGreen1,
    DarkSeaGreen1a,
    DarkSeaGreen2,
    DarkSeaGreen2a,
    DarkSeaGreen3,
    DarkSeaGreen3a,
    DarkSeaGreen4,
    DarkSeaGreen4a,
    DarkSeaGreen,
    DarkSlateGray1,
    DarkSlateGray2,
    DarkSlateGray3,
    DarkTurquoise,
    DarkViolet,
    DarkVioleta,
    DeepPink1,
    DeepPink1a,
    DeepPink2,
    DeepPink3,
    DeepPink3a,
    DeepPink4,
    DeepPink4a,
    DeepPink4b,
    DeepSkyBlue1,
    DeepSkyBlue2,
    DeepSkyBlue3,
    DeepSkyBlue3a,
    DeepSkyBlue4,
    DeepSkyBlue4a,
    DeepSkyBlue4b,
    DodgerBlue1,
    DodgerBlue2,
    DodgerBlue3,
    Fuchsia,
    Gold1,
    Gold3,
    Gold3a,
    Green1,
    Green3,
    Green3a,
    Green4,
    GreenYellow,
    Green,
    Grey0,
    Grey100,
    Grey11,
    Grey15,
    Grey19,
    Grey23,
    Grey27,
    Grey30,
    Grey35,
    Grey37,
    Grey39,
    Grey3,
    Grey42,
    Grey46,
    Grey50,
    Grey53,
    Grey54,
    Grey58,
    Grey62,
    Grey63,
    Grey66,
    Grey69,
    Grey70,
    Grey74,
    Grey78,
    Grey7,
    Grey82,
    Grey84,
    Grey85,
    Grey89,
    Grey93,
    Grey,
    Honeydew2,
    HotPink2,
    HotPink3,
    HotPink3a,
    HotPink,
    HotPinka,
    IndianRed1,
    IndianRed1a,
    IndianRed,
    IndianReda,
    Khaki1,
    Khaki3,
    LightCoral,
    LightCyan1,
    LightCyan3,
    LightGoldenrod1,
    LightGoldenrod2,
    LightGoldenrod2a,
    LightGoldenrod2b,
    LightGoldenrod3,
    LightGreen,
    LightGreena,
    LightPink1,
    LightPink3,
    LightPink4,
    LightSalmon1,
    LightSalmon3,
    LightSalmon3a,
    LightSeaGreen,
    LightSkyBlue1,
    LightSkyBlue3,
    LightSkyBlue3a,
    LightSlateBlue,
    LightSlateGrey,
    LightSteelBlue1,
    LightSteelBlue3,
    LightSteelBlue,
    LightYellow,
    Lime,
    Magenta,
    Magenta1,
    Magenta2,
    Magenta3,
    Magenta3a,
    Magenta3b,
    Maroon,
    MediumOrchid1,
    MediumOrchid1a,
    MediumOrchid3,
    MediumOrchid,
    MediumPurple1,
    MediumPurple2,
    MediumPurple2a,
    MediumPurple3,
    MediumPurple3a,
    MediumPurple4,
    MediumPurple,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MistyRose1,
    MistyRose3,
    NavajoWhite1,
    NavajoWhite3,
    NavyBlue,
    Navy,
    Olive,
    Orange,
    Orange1,
    Orange2,
    Orange3,
    OrangeRed1,
    Orchid1,
    Orchid2,
    Orchid,
    PaleGreen1,
    PaleGreen1a,
    PaleGreen3,
    PaleGreen3a,
    PaleTurquoise1,
    PaleTurquoise4,
    PaleVioletRed1,
    Pink1,
    Pink3,
    Plum1,
    Plum2,
    Plum3,
    Plum4,
    Purple3,
    Purple4,
    Purple4a,
    Purple,
    Purplea,
    Purpleb,
    Red1,
    Red3,
    Red3a,
    Red,
    RosyBrown,
    RoyalBlue1,
    Salmon1,
    SandyBrown,
    SeaGreen1,
    SeaGreen1a,
    SeaGreen2,
    SeaGreen3,
    Silver,
    SkyBlue1,
    SkyBlue2,
    SkyBlue3,
    SlateBlue1,
    SlateBlue3,
    SlateBlue3a,
    SpringGreen1,
    SpringGreen2,
    SpringGreen2a,
    SpringGreen3,
    SpringGreen3a,
    SpringGreen4,
    SteelBlue1,
    SteelBlue1a,
    SteelBlue3,
    SteelBlue,
    Tan,
    Teal,
    Thistle1,
    Thistle3,
    Turquoise2,
    Turquoise4,
    Violet,
    Wheat1,
    Wheat4,
    White,
    Yellow1,
    Yellow2,
    Yellow3,
    Yellow3a,
    Yellow4,
    Yellow4a,
    Yellow,
}

pub const NORMAL: &str = "\\x1b[0;39m";
pub const AQUA: &str = "\\x1b[38;5;014m";
pub const AQUAMARINE1: &str = "\\x1b[38;5;086m";
pub const AQUAMARINE1A: &str = "\\x1b[38;5;122m";
pub const AQUAMARINE3: &str = "\\x1b[38;5;079m";
pub const BLACK: &str = "\\x1b[38;5;000m";
pub const BLUE1: &str = "\\x1b[38;5;021m";
pub const BLUE3: &str = "\\x1b[38;5;019m";
pub const BLUE3A: &str = "\\x1b[38;5;020m";
pub const BLUEVIOLET: &str = "\\x1b[38;5;057m";
pub const BLUE: &str = "\\x1b[38;5;012m";
pub const CADETBLUE: &str = "\\x1b[38;5;072m";
pub const CADETBLUEA: &str = "\\x1b[38;5;073m";
pub const CHARTREUSE1: &str = "\\x1b[38;5;118m";
pub const CHARTREUSE2: &str = "\\x1b[38;5;082m";
pub const CHARTREUSE2A: &str = "\\x1b[38;5;112m";
pub const CHARTREUSE3: &str = "\\x1b[38;5;070m";
pub const CHARTREUSE3A: &str = "\\x1b[38;5;076m";
pub const CHARTREUSE4: &str = "\\x1b[38;5;064m";
pub const CORNFLOWERBLUE: &str = "\\x1b[38;5;069m";
pub const CORNSILK1: &str = "\\x1b[38;5;230m";
pub const CYAN1: &str = "\\x1b[38;5;051m";
pub const CYAN2: &str = "\\x1b[38;5;050m";
pub const CYAN: &str = "\\x1b[38;5;043m";
pub const DARKBLUE: &str = "\\x1b[38;5;018m";
pub const DARKCYAN: &str = "\\x1b[38;5;036m";
pub const DARKGOLDENROD: &str = "\\x1b[38;5;136m";
pub const DARKGREEN: &str = "\\x1b[38;5;022m";
pub const DARKKHAKI: &str = "\\x1b[38;5;143m";
pub const DARKMAGENTA: &str = "\\x1b[38;5;090m";
pub const DARKMAGENTAA: &str = "\\x1b[38;5;091m";
pub const DARKOLIVEGREEN1: &str = "\\x1b[38;5;191m";
pub const DARKOLIVEGREEN1A: &str = "\\x1b[38;5;192m";
pub const DARKOLIVEGREEN2: &str = "\\x1b[38;5;155m";
pub const DARKOLIVEGREEN3: &str = "\\x1b[38;5;107m";
pub const DARKOLIVEGREEN3A: &str = "\\x1b[38;5;113m";
pub const DARKOLIVEGREEN3B: &str = "\\x1b[38;5;149m";
pub const DARKORANGE3: &str = "\\x1b[38;5;130m";
pub const DARKORANGE3A: &str = "\\x1b[38;5;166m";
pub const DARKORANGE: &str = "\\x1b[38;5;208m";
pub const DARKRED: &str = "\\x1b[38;5;052m";
pub const DARKREDA: &str = "\\x1b[38;5;088m";
pub const DARKSEAGREEN1: &str = "\\x1b[38;5;158m";
pub const DARKSEAGREEN1A: &str = "\\x1b[38;5;193m";
pub const DARKSEAGREEN2: &str = "\\x1b[38;5;151m";
pub const DARKSEAGREEN2A: &str = "\\x1b[38;5;157m";
pub const DARKSEAGREEN3: &str = "\\x1b[38;5;115m";
pub const DARKSEAGREEN3A: &str = "\\x1b[38;5;150m";
pub const DARKSEAGREEN4: &str = "\\x1b[38;5;065m";
pub const DARKSEAGREEN4A: &str = "\\x1b[38;5;071m";
pub const DARKSEAGREEN: &str = "\\x1b[38;5;108m";
pub const DARKSLATEGRAY1: &str = "\\x1b[38;5;123m";
pub const DARKSLATEGRAY2: &str = "\\x1b[38;5;087m";
pub const DARKSLATEGRAY3: &str = "\\x1b[38;5;116m";
pub const DARKTURQUOISE: &str = "\\x1b[38;5;044m";
pub const DARKVIOLET: &str = "\\x1b[38;5;092m";
pub const DARKVIOLETA: &str = "\\x1b[38;5;128m";
pub const DEEPPINK1: &str = "\\x1b[38;5;198m";
pub const DEEPPINK1A: &str = "\\x1b[38;5;199m";
pub const DEEPPINK2: &str = "\\x1b[38;5;197m";
pub const DEEPPINK3: &str = "\\x1b[38;5;161m";
pub const DEEPPINK3A: &str = "\\x1b[38;5;162m";
pub const DEEPPINK4: &str = "\\x1b[38;5;053m";
pub const DEEPPINK4A: &str = "\\x1b[38;5;089m";
pub const DEEPPINK4B: &str = "\\x1b[38;5;125m";
pub const DEEPSKYBLUE1: &str = "\\x1b[38;5;039m";
pub const DEEPSKYBLUE2: &str = "\\x1b[38;5;038m";
pub const DEEPSKYBLUE3: &str = "\\x1b[38;5;031m";
pub const DEEPSKYBLUE3A: &str = "\\x1b[38;5;032m";
pub const DEEPSKYBLUE4: &str = "\\x1b[38;5;023m";
pub const DEEPSKYBLUE4A: &str = "\\x1b[38;5;024m";
pub const DEEPSKYBLUE4B: &str = "\\x1b[38;5;025m";
pub const DODGERBLUE1: &str = "\\x1b[38;5;033m";
pub const DODGERBLUE2: &str = "\\x1b[38;5;027m";
pub const DODGERBLUE3: &str = "\\x1b[38;5;026m";
pub const FUCHSIA: &str = "\\x1b[38;5;013m";
pub const GOLD1: &str = "\\x1b[38;5;220m";
pub const GOLD3: &str = "\\x1b[38;5;142m";
pub const GOLD3A: &str = "\\x1b[38;5;178m";
pub const GREEN1: &str = "\\x1b[38;5;046m";
pub const GREEN3: &str = "\\x1b[38;5;034m";
pub const GREEN3A: &str = "\\x1b[38;5;040m";
pub const GREEN4: &str = "\\x1b[38;5;028m";
pub const GREENYELLOW: &str = "\\x1b[38;5;154m";
pub const GREEN: &str = "\\x1b[38;5;002m";
pub const GREY0: &str = "\\x1b[38;5;016m";
pub const GREY100: &str = "\\x1b[38;5;231m";
pub const GREY11: &str = "\\x1b[38;5;234m";
pub const GREY15: &str = "\\x1b[38;5;235m";
pub const GREY19: &str = "\\x1b[38;5;236m";
pub const GREY23: &str = "\\x1b[38;5;237m";
pub const GREY27: &str = "\\x1b[38;5;238m";
pub const GREY30: &str = "\\x1b[38;5;239m";
pub const GREY35: &str = "\\x1b[38;5;240m";
pub const GREY37: &str = "\\x1b[38;5;059m";
pub const GREY39: &str = "\\x1b[38;5;241m";
pub const GREY3: &str = "\\x1b[38;5;232m";
pub const GREY42: &str = "\\x1b[38;5;242m";
pub const GREY46: &str = "\\x1b[38;5;243m";
pub const GREY50: &str = "\\x1b[38;5;244m";
pub const GREY53: &str = "\\x1b[38;5;102m";
pub const GREY54: &str = "\\x1b[38;5;245m";
pub const GREY58: &str = "\\x1b[38;5;246m";
pub const GREY62: &str = "\\x1b[38;5;247m";
pub const GREY63: &str = "\\x1b[38;5;139m";
pub const GREY66: &str = "\\x1b[38;5;248m";
pub const GREY69: &str = "\\x1b[38;5;145m";
pub const GREY70: &str = "\\x1b[38;5;249m";
pub const GREY74: &str = "\\x1b[38;5;250m";
pub const GREY78: &str = "\\x1b[38;5;251m";
pub const GREY7: &str = "\\x1b[38;5;233m";
pub const GREY82: &str = "\\x1b[38;5;252m";
pub const GREY84: &str = "\\x1b[38;5;188m";
pub const GREY85: &str = "\\x1b[38;5;253m";
pub const GREY89: &str = "\\x1b[38;5;254m";
pub const GREY93: &str = "\\x1b[38;5;255m";
pub const GREY: &str = "\\x1b[38;5;008m";
pub const HONEYDEW2: &str = "\\x1b[38;5;194m";
pub const HOTPINK2: &str = "\\x1b[38;5;169m";
pub const HOTPINK3: &str = "\\x1b[38;5;132m";
pub const HOTPINK3A: &str = "\\x1b[38;5;168m";
pub const HOTPINK: &str = "\\x1b[38;5;205m";
pub const HOTPINKA: &str = "\\x1b[38;5;206m";
pub const INDIANRED1: &str = "\\x1b[38;5;203m";
pub const INDIANRED1A: &str = "\\x1b[38;5;204m";
pub const INDIANRED: &str = "\\x1b[38;5;131m";
pub const INDIANREDA: &str = "\\x1b[38;5;167m";
pub const KHAKI1: &str = "\\x1b[38;5;228m";
pub const KHAKI3: &str = "\\x1b[38;5;185m";
pub const LIGHTCORAL: &str = "\\x1b[38;5;210m";
pub const LIGHTCYAN1: &str = "\\x1b[38;5;195m";
pub const LIGHTCYAN3: &str = "\\x1b[38;5;152m";
pub const LIGHTGOLDENROD1: &str = "\\x1b[38;5;227m";
pub const LIGHTGOLDENROD2: &str = "\\x1b[38;5;186m";
pub const LIGHTGOLDENROD2A: &str = "\\x1b[38;5;221m";
pub const LIGHTGOLDENROD2B: &str = "\\x1b[38;5;222m";
pub const LIGHTGOLDENROD3: &str = "\\x1b[38;5;179m";
pub const LIGHTGREEN: &str = "\\x1b[38;5;119m";
pub const LIGHTGREENA: &str = "\\x1b[38;5;120m";
pub const LIGHTPINK1: &str = "\\x1b[38;5;217m";
pub const LIGHTPINK3: &str = "\\x1b[38;5;174m";
pub const LIGHTPINK4: &str = "\\x1b[38;5;095m";
pub const LIGHTSALMON1: &str = "\\x1b[38;5;216m";
pub const LIGHTSALMON3: &str = "\\x1b[38;5;137m";
pub const LIGHTSALMON3A: &str = "\\x1b[38;5;173m";
pub const LIGHTSEAGREEN: &str = "\\x1b[38;5;037m";
pub const LIGHTSKYBLUE1: &str = "\\x1b[38;5;153m";
pub const LIGHTSKYBLUE3: &str = "\\x1b[38;5;109m";
pub const LIGHTSKYBLUE3A: &str = "\\x1b[38;5;110m";
pub const LIGHTSLATEBLUE: &str = "\\x1b[38;5;105m";
pub const LIGHTSLATEGREY: &str = "\\x1b[38;5;103m";
pub const LIGHTSTEELBLUE1: &str = "\\x1b[38;5;189m";
pub const LIGHTSTEELBLUE3: &str = "\\x1b[38;5;146m";
pub const LIGHTSTEELBLUE: &str = "\\x1b[38;5;147m";
pub const LIGHTYELLOW: &str = "\\x1b[38;5;187m";
pub const LIME: &str = "\\x1b[38;5;010m";
pub const MAGENTA: &str = "\\x1b[38;5;201m";
pub const MAGENTA1: &str = "\\x1b[38;5;165m";
pub const MAGENTA2: &str = "\\x1b[38;5;200m";
pub const MAGENTA3: &str = "\\x1b[38;5;127m";
pub const MAGENTA3A: &str = "\\x1b[38;5;163m";
pub const MAGENTA3B: &str = "\\x1b[38;5;164m";
pub const MAROON: &str = "\\x1b[38;5;001m";
pub const MEDIUMORCHID1: &str = "\\x1b[38;5;171m";
pub const MEDIUMORCHID1A: &str = "\\x1b[38;5;207m";
pub const MEDIUMORCHID3: &str = "\\x1b[38;5;133m";
pub const MEDIUMORCHID: &str = "\\x1b[38;5;134m";
pub const MEDIUMPURPLE1: &str = "\\x1b[38;5;141m";
pub const MEDIUMPURPLE2: &str = "\\x1b[38;5;135m";
pub const MEDIUMPURPLE2A: &str = "\\x1b[38;5;140m";
pub const MEDIUMPURPLE3: &str = "\\x1b[38;5;097m";
pub const MEDIUMPURPLE3A: &str = "\\x1b[38;5;098m";
pub const MEDIUMPURPLE4: &str = "\\x1b[38;5;060m";
pub const MEDIUMPURPLE: &str = "\\x1b[38;5;104m";
pub const MEDIUMSPRINGGREEN: &str = "\\x1b[38;5;049m";
pub const MEDIUMTURQUOISE: &str = "\\x1b[38;5;080m";
pub const MEDIUMVIOLETRED: &str = "\\x1b[38;5;126m";
pub const MISTYROSE1: &str = "\\x1b[38;5;224m";
pub const MISTYROSE3: &str = "\\x1b[38;5;181m";
pub const NAVAJOWHITE1: &str = "\\x1b[38;5;223m";
pub const NAVAJOWHITE3: &str = "\\x1b[38;5;144m";
pub const NAVYBLUE: &str = "\\x1b[38;5;017m";
pub const NAVY: &str = "\\x1b[38;5;004m";
pub const OLIVE: &str = "\\x1b[38;5;003m";
pub const ORANGE: &str = "\\x1b[38;5;214m";
pub const ORANGE1: &str = "\\x1b[38;5;172m";
pub const ORANGE2: &str = "\\x1b[38;5;058m";
pub const ORANGE3: &str = "\\x1b[38;5;094m";
pub const ORANGERED1: &str = "\\x1b[38;5;202m";
pub const ORCHID1: &str = "\\x1b[38;5;213m";
pub const ORCHID2: &str = "\\x1b[38;5;212m";
pub const ORCHID: &str = "\\x1b[38;5;170m";
pub const PALEGREEN1: &str = "\\x1b[38;5;121m";
pub const PALEGREEN1A: &str = "\\x1b[38;5;156m";
pub const PALEGREEN3: &str = "\\x1b[38;5;077m";
pub const PALEGREEN3A: &str = "\\x1b[38;5;114m";
pub const PALETURQUOISE1: &str = "\\x1b[38;5;159m";
pub const PALETURQUOISE4: &str = "\\x1b[38;5;066m";
pub const PALEVIOLETRED1: &str = "\\x1b[38;5;211m";
pub const PINK1: &str = "\\x1b[38;5;218m";
pub const PINK3: &str = "\\x1b[38;5;175m";
pub const PLUM1: &str = "\\x1b[38;5;219m";
pub const PLUM2: &str = "\\x1b[38;5;183m";
pub const PLUM3: &str = "\\x1b[38;5;176m";
pub const PLUM4: &str = "\\x1b[38;5;096m";
pub const PURPLE3: &str = "\\x1b[38;5;056m";
pub const PURPLE4: &str = "\\x1b[38;5;054m";
pub const PURPLE4A: &str = "\\x1b[38;5;055m";
pub const PURPLE: &str = "\\x1b[38;5;005m";
pub const PURPLEA: &str = "\\x1b[38;5;093m";
pub const PURPLEB: &str = "\\x1b[38;5;129m";
pub const RED1: &str = "\\x1b[38;5;196m";
pub const RED3: &str = "\\x1b[38;5;124m";
pub const RED3A: &str = "\\x1b[38;5;160m";
pub const RED: &str = "\\x1b[38;5;009m";
pub const ROSYBROWN: &str = "\\x1b[38;5;138m";
pub const ROYALBLUE1: &str = "\\x1b[38;5;063m";
pub const SALMON1: &str = "\\x1b[38;5;209m";
pub const SANDYBROWN: &str = "\\x1b[38;5;215m";
pub const SEAGREEN1: &str = "\\x1b[38;5;084m";
pub const SEAGREEN1A: &str = "\\x1b[38;5;085m";
pub const SEAGREEN2: &str = "\\x1b[38;5;083m";
pub const SEAGREEN3: &str = "\\x1b[38;5;078m";
pub const SILVER: &str = "\\x1b[38;5;007m";
pub const SKYBLUE1: &str = "\\x1b[38;5;117m";
pub const SKYBLUE2: &str = "\\x1b[38;5;111m";
pub const SKYBLUE3: &str = "\\x1b[38;5;074m";
pub const SLATEBLUE1: &str = "\\x1b[38;5;099m";
pub const SLATEBLUE3: &str = "\\x1b[38;5;061m";
pub const SLATEBLUE3A: &str = "\\x1b[38;5;062m";
pub const SPRINGGREEN1: &str = "\\x1b[38;5;048m";
pub const SPRINGGREEN2: &str = "\\x1b[38;5;042m";
pub const SPRINGGREEN2A: &str = "\\x1b[38;5;047m";
pub const SPRINGGREEN3: &str = "\\x1b[38;5;035m";
pub const SPRINGGREEN3A: &str = "\\x1b[38;5;041m";
pub const SPRINGGREEN4: &str = "\\x1b[38;5;029m";
pub const STEELBLUE1: &str = "\\x1b[38;5;075m";
pub const STEELBLUE1A: &str = "\\x1b[38;5;081m";
pub const STEELBLUE3: &str = "\\x1b[38;5;068m";
pub const STEELBLUE: &str = "\\x1b[38;5;067m";
pub const TAN: &str = "\\x1b[38;5;180m";
pub const TEAL: &str = "\\x1b[38;5;006m";
pub const THISTLE1: &str = "\\x1b[38;5;225m";
pub const THISTLE3: &str = "\\x1b[38;5;182m";
pub const TURQUOISE2: &str = "\\x1b[38;5;045m";
pub const TURQUOISE4: &str = "\\x1b[38;5;030m";
pub const VIOLET: &str = "\\x1b[38;5;177m";
pub const WHEAT1: &str = "\\x1b[38;5;229m";
pub const WHEAT4: &str = "\\x1b[38;5;101m";
pub const WHITE: &str = "\\x1b[38;5;015m";
pub const YELLOW1: &str = "\\x1b[38;5;226m";
pub const YELLOW2: &str = "\\x1b[38;5;190m";
pub const YELLOW3: &str = "\\x1b[38;5;148m";
pub const YELLOW3A: &str = "\\x1b[38;5;184m";
pub const YELLOW4: &str = "\\x1b[38;5;100m";
pub const YELLOW4A: &str = "\\x1b[38;5;106m";
pub const YELLOW: &str = "\\x1b[38;5;011m";

