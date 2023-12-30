// color_map.rs
use crate::colors::{Colors,
                    NORMAL,
                    AQUA,
                    AQUAMARINE1,
                    AQUAMARINE1A,
                    AQUAMARINE3,
                    BLACK,
                    BLUE1,
                    BLUE3,
                    BLUE3A,
                    BLUEVIOLET,
                    BLUE,
                    CADETBLUE,
                    CADETBLUEA,
                    CHARTREUSE1,
                    CHARTREUSE2,
                    CHARTREUSE2A,
                    CHARTREUSE3,
                    CHARTREUSE3A,
                    CHARTREUSE4,
                    CORNFLOWERBLUE,
                    CORNSILK1,
                    CYAN1,
                    CYAN2,
                    CYAN,
                    DARKBLUE,
                    DARKCYAN,
                    DARKGOLDENROD,
                    DARKGREEN,
                    DARKKHAKI,
                    DARKMAGENTA,
                    DARKMAGENTAA,
                    DARKOLIVEGREEN1,
                    DARKOLIVEGREEN1A,
                    DARKOLIVEGREEN2,
                    DARKOLIVEGREEN3,
                    DARKOLIVEGREEN3A,
                    DARKOLIVEGREEN3B,
                    DARKORANGE3,
                    DARKORANGE3A,
                    DARKORANGE,
                    DARKRED,
                    DARKREDA,
                    DARKSEAGREEN1,
                    DARKSEAGREEN1A,
                    DARKSEAGREEN2,
                    DARKSEAGREEN2A,
                    DARKSEAGREEN3,
                    DARKSEAGREEN3A,
                    DARKSEAGREEN4,
                    DARKSEAGREEN4A,
                    DARKSEAGREEN,
                    DARKSLATEGRAY1,
                    DARKSLATEGRAY2,
                    DARKSLATEGRAY3,
                    DARKTURQUOISE,
                    DARKVIOLET,
                    DARKVIOLETA,
                    DEEPPINK1,
                    DEEPPINK1A,
                    DEEPPINK2,
                    DEEPPINK3,
                    DEEPPINK3A,
                    DEEPPINK4,
                    DEEPPINK4A,
                    DEEPPINK4B,
                    DEEPSKYBLUE1,
                    DEEPSKYBLUE2,
                    DEEPSKYBLUE3,
                    DEEPSKYBLUE3A,
                    DEEPSKYBLUE4,
                    DEEPSKYBLUE4A,
                    DEEPSKYBLUE4B,
                    DODGERBLUE1,
                    DODGERBLUE2,
                    DODGERBLUE3,
                    FUCHSIA,
                    GOLD1,
                    GOLD3,
                    GOLD3A,
                    GREEN1,
                    GREEN3,
                    GREEN3A,
                    GREEN4,
                    GREENYELLOW,
                    GREEN,
                    GREY0,
                    GREY100,
                    GREY11,
                    GREY15,
                    GREY19,
                    GREY23,
                    GREY27,
                    GREY30,
                    GREY35,
                    GREY37,
                    GREY39,
                    GREY3,
                    GREY42,
                    GREY46,
                    GREY50,
                    GREY53,
                    GREY54,
                    GREY58,
                    GREY62,
                    GREY63,
                    GREY66,
                    GREY69,
                    GREY70,
                    GREY74,
                    GREY78,
                    GREY7,
                    GREY82,
                    GREY84,
                    GREY85,
                    GREY89,
                    GREY93,
                    GREY,
                    HONEYDEW2,
                    HOTPINK2,
                    HOTPINK3,
                    HOTPINK3A,
                    HOTPINK,
                    HOTPINKA,
                    INDIANRED1,
                    INDIANRED1A,
                    INDIANRED,
                    INDIANREDA,
                    KHAKI1,
                    KHAKI3,
                    LIGHTCORAL,
                    LIGHTCYAN1,
                    LIGHTCYAN3,
                    LIGHTGOLDENROD1,
                    LIGHTGOLDENROD2,
                    LIGHTGOLDENROD2A,
                    LIGHTGOLDENROD2B,
                    LIGHTGOLDENROD3,
                    LIGHTGREEN,
                    LIGHTGREENA,
                    LIGHTPINK1,
                    LIGHTPINK3,
                    LIGHTPINK4,
                    LIGHTSALMON1,
                    LIGHTSALMON3,
                    LIGHTSALMON3A,
                    LIGHTSEAGREEN,
                    LIGHTSKYBLUE1,
                    LIGHTSKYBLUE3,
                    LIGHTSKYBLUE3A,
                    LIGHTSLATEBLUE,
                    LIGHTSLATEGREY,
                    LIGHTSTEELBLUE1,
                    LIGHTSTEELBLUE3,
                    LIGHTSTEELBLUE,
                    LIGHTYELLOW,
                    LIME,
                    MAGENTA,
                    MAGENTA1,
                    MAGENTA2,
                    MAGENTA3,
                    MAGENTA3A,
                    MAGENTA3B,
                    MAROON,
                    MEDIUMORCHID1,
                    MEDIUMORCHID1A,
                    MEDIUMORCHID3,
                    MEDIUMORCHID,
                    MEDIUMPURPLE1,
                    MEDIUMPURPLE2,
                    MEDIUMPURPLE2A,
                    MEDIUMPURPLE3,
                    MEDIUMPURPLE3A,
                    MEDIUMPURPLE4,
                    MEDIUMPURPLE,
                    MEDIUMSPRINGGREEN,
                    MEDIUMTURQUOISE,
                    MEDIUMVIOLETRED,
                    MISTYROSE1,
                    MISTYROSE3,
                    NAVAJOWHITE1,
                    NAVAJOWHITE3,
                    NAVYBLUE,
                    NAVY,
                    OLIVE,
                    ORANGE,
                    ORANGE1,
                    ORANGE2,
                    ORANGE3,
                    ORANGERED1,
                    ORCHID1,
                    ORCHID2,
                    ORCHID,
                    PALEGREEN1,
                    PALEGREEN1A,
                    PALEGREEN3,
                    PALEGREEN3A,
                    PALETURQUOISE1,
                    PALETURQUOISE4,
                    PALEVIOLETRED1,
                    PINK1,
                    PINK3,
                    PLUM1,
                    PLUM2,
                    PLUM3,
                    PLUM4,
                    PURPLE3,
                    PURPLE4,
                    PURPLE4A,
                    PURPLE,
                    PURPLEA,
                    PURPLEB,
                    RED1,
                    RED3,
                    RED3A,
                    RED,
                    ROSYBROWN,
                    ROYALBLUE1,
                    SALMON1,
                    SANDYBROWN,
                    SEAGREEN1,
                    SEAGREEN1A,
                    SEAGREEN2,
                    SEAGREEN3,
                    SILVER,
                    SKYBLUE1,
                    SKYBLUE2,
                    SKYBLUE3,
                    SLATEBLUE1,
                    SLATEBLUE3,
                    SLATEBLUE3A,
                    SPRINGGREEN1,
                    SPRINGGREEN2,
                    SPRINGGREEN2A,
                    SPRINGGREEN3,
                    SPRINGGREEN3A,
                    SPRINGGREEN4,
                    STEELBLUE1,
                    STEELBLUE1A,
                    STEELBLUE3,
                    STEELBLUE,
                    TAN,
                    TEAL,
                    THISTLE1,
                    THISTLE3,
                    TURQUOISE2,
                    TURQUOISE4,
                    VIOLET,
                    WHEAT1,
                    WHEAT4,
                    WHITE,
                    YELLOW1,
                    YELLOW2,
                    YELLOW3,
                    YELLOW3A,
                    YELLOW4,
                    YELLOW4A,
                    YELLOW};
use std::collections::HashMap;

pub fn initialize_color_map() -> HashMap<Colors, &'static str> {
    let mut color_map = HashMap::new();
    color_map.insert(Colors::Normal, NORMAL);
    color_map.insert(Colors::Aqua, AQUA);
    color_map.insert(Colors::Aquamarine1, AQUAMARINE1);
    color_map.insert(Colors::Aquamarine1a, AQUAMARINE1A);
    color_map.insert(Colors::Aquamarine3, AQUAMARINE3);
    color_map.insert(Colors::Black, BLACK);
    color_map.insert(Colors::Blue1, BLUE1);
    color_map.insert(Colors::Blue3, BLUE3);
    color_map.insert(Colors::Blue3a, BLUE3A);
    color_map.insert(Colors::BlueViolet, BLUEVIOLET);
    color_map.insert(Colors::Blue, BLUE);
    color_map.insert(Colors::CadetBlue, CADETBLUE);
    color_map.insert(Colors::CadetBluea, CADETBLUEA);
    color_map.insert(Colors::Chartreuse1, CHARTREUSE1);
    color_map.insert(Colors::Chartreuse2, CHARTREUSE2);
    color_map.insert(Colors::Chartreuse2a, CHARTREUSE2A);
    color_map.insert(Colors::Chartreuse3, CHARTREUSE3);
    color_map.insert(Colors::Chartreuse3a, CHARTREUSE3A);
    color_map.insert(Colors::Chartreuse4, CHARTREUSE4);
    color_map.insert(Colors::CornflowerBlue, CORNFLOWERBLUE);
    color_map.insert(Colors::Cornsilk1, CORNSILK1);
    color_map.insert(Colors::Cyan1, CYAN1);
    color_map.insert(Colors::Cyan2, CYAN2);
    color_map.insert(Colors::Cyan, CYAN);
    color_map.insert(Colors::DarkBlue, DARKBLUE);
    color_map.insert(Colors::DarkCyan, DARKCYAN);
    color_map.insert(Colors::DarkGoldenrod, DARKGOLDENROD);
    color_map.insert(Colors::DarkGreen, DARKGREEN);
    color_map.insert(Colors::DarkKhaki, DARKKHAKI);
    color_map.insert(Colors::DarkMagenta, DARKMAGENTA);
    color_map.insert(Colors::DarkMagentaa, DARKMAGENTAA);
    color_map.insert(Colors::DarkOliveGreen1, DARKOLIVEGREEN1);
    color_map.insert(Colors::DarkOliveGreen1a, DARKOLIVEGREEN1A);
    color_map.insert(Colors::DarkOliveGreen2, DARKOLIVEGREEN2);
    color_map.insert(Colors::DarkOliveGreen3, DARKOLIVEGREEN3);
    color_map.insert(Colors::DarkOliveGreen3a, DARKOLIVEGREEN3A);
    color_map.insert(Colors::DarkOliveGreen3b, DARKOLIVEGREEN3B);
    color_map.insert(Colors::DarkOrange3, DARKORANGE3);
    color_map.insert(Colors::DarkOrange3a, DARKORANGE3A);
    color_map.insert(Colors::DarkOrange, DARKORANGE);
    color_map.insert(Colors::DarkRed, DARKRED);
    color_map.insert(Colors::DarkReda, DARKREDA);
    color_map.insert(Colors::DarkSeaGreen1, DARKSEAGREEN1);
    color_map.insert(Colors::DarkSeaGreen1a, DARKSEAGREEN1A);
    color_map.insert(Colors::DarkSeaGreen2, DARKSEAGREEN2);
    color_map.insert(Colors::DarkSeaGreen2a, DARKSEAGREEN2A);
    color_map.insert(Colors::DarkSeaGreen3, DARKSEAGREEN3);
    color_map.insert(Colors::DarkSeaGreen3a, DARKSEAGREEN3A);
    color_map.insert(Colors::DarkSeaGreen4, DARKSEAGREEN4);
    color_map.insert(Colors::DarkSeaGreen4a, DARKSEAGREEN4A);
    color_map.insert(Colors::DarkSeaGreen, DARKSEAGREEN);
    color_map.insert(Colors::DarkSlateGray1, DARKSLATEGRAY1);
    color_map.insert(Colors::DarkSlateGray2, DARKSLATEGRAY2);
    color_map.insert(Colors::DarkSlateGray3, DARKSLATEGRAY3);
    color_map.insert(Colors::DarkTurquoise, DARKTURQUOISE);
    color_map.insert(Colors::DarkViolet, DARKVIOLET);
    color_map.insert(Colors::DarkVioleta, DARKVIOLETA);
    color_map.insert(Colors::DeepPink1, DEEPPINK1);
    color_map.insert(Colors::DeepPink1a, DEEPPINK1A);
    color_map.insert(Colors::DeepPink2, DEEPPINK2);
    color_map.insert(Colors::DeepPink3, DEEPPINK3);
    color_map.insert(Colors::DeepPink3a, DEEPPINK3A);
    color_map.insert(Colors::DeepPink4, DEEPPINK4);
    color_map.insert(Colors::DeepPink4a, DEEPPINK4A);
    color_map.insert(Colors::DeepPink4b, DEEPPINK4B);
    color_map.insert(Colors::DeepSkyBlue1, DEEPSKYBLUE1);
    color_map.insert(Colors::DeepSkyBlue2, DEEPSKYBLUE2);
    color_map.insert(Colors::DeepSkyBlue3, DEEPSKYBLUE3);
    color_map.insert(Colors::DeepSkyBlue3a, DEEPSKYBLUE3A);
    color_map.insert(Colors::DeepSkyBlue4, DEEPSKYBLUE4);
    color_map.insert(Colors::DeepSkyBlue4a, DEEPSKYBLUE4A);
    color_map.insert(Colors::DeepSkyBlue4b, DEEPSKYBLUE4B);
    color_map.insert(Colors::DodgerBlue1, DODGERBLUE1);
    color_map.insert(Colors::DodgerBlue2, DODGERBLUE2);
    color_map.insert(Colors::DodgerBlue3, DODGERBLUE3);
    color_map.insert(Colors::Fuchsia, FUCHSIA);
    color_map.insert(Colors::Gold1, GOLD1);
    color_map.insert(Colors::Gold3, GOLD3);
    color_map.insert(Colors::Gold3a, GOLD3A);
    color_map.insert(Colors::Green1, GREEN1);
    color_map.insert(Colors::Green3, GREEN3);
    color_map.insert(Colors::Green3a, GREEN3A);
    color_map.insert(Colors::Green4, GREEN4);
    color_map.insert(Colors::GreenYellow, GREENYELLOW);
    color_map.insert(Colors::Green, GREEN);
    color_map.insert(Colors::Grey0, GREY0);
    color_map.insert(Colors::Grey100, GREY100);
    color_map.insert(Colors::Grey11, GREY11);
    color_map.insert(Colors::Grey15, GREY15);
    color_map.insert(Colors::Grey19, GREY19);
    color_map.insert(Colors::Grey23, GREY23);
    color_map.insert(Colors::Grey27, GREY27);
    color_map.insert(Colors::Grey30, GREY30);
    color_map.insert(Colors::Grey35, GREY35);
    color_map.insert(Colors::Grey37, GREY37);
    color_map.insert(Colors::Grey39, GREY39);
    color_map.insert(Colors::Grey3, GREY3);
    color_map.insert(Colors::Grey42, GREY42);
    color_map.insert(Colors::Grey46, GREY46);
    color_map.insert(Colors::Grey50, GREY50);
    color_map.insert(Colors::Grey53, GREY53);
    color_map.insert(Colors::Grey54, GREY54);
    color_map.insert(Colors::Grey58, GREY58);
    color_map.insert(Colors::Grey62, GREY62);
    color_map.insert(Colors::Grey63, GREY63);
    color_map.insert(Colors::Grey66, GREY66);
    color_map.insert(Colors::Grey69, GREY69);
    color_map.insert(Colors::Grey70, GREY70);
    color_map.insert(Colors::Grey74, GREY74);
    color_map.insert(Colors::Grey78, GREY78);
    color_map.insert(Colors::Grey7, GREY7);
    color_map.insert(Colors::Grey82, GREY82);
    color_map.insert(Colors::Grey84, GREY84);
    color_map.insert(Colors::Grey85, GREY85);
    color_map.insert(Colors::Grey89, GREY89);
    color_map.insert(Colors::Grey93, GREY93);
    color_map.insert(Colors::Grey, GREY);
    color_map.insert(Colors::Honeydew2, HONEYDEW2);
    color_map.insert(Colors::HotPink2, HOTPINK2);
    color_map.insert(Colors::HotPink3, HOTPINK3);
    color_map.insert(Colors::HotPink3a, HOTPINK3A);
    color_map.insert(Colors::HotPink, HOTPINK);
    color_map.insert(Colors::HotPinka, HOTPINKA);
    color_map.insert(Colors::IndianRed1, INDIANRED1);
    color_map.insert(Colors::IndianRed1a, INDIANRED1A);
    color_map.insert(Colors::IndianRed, INDIANRED);
    color_map.insert(Colors::IndianReda, INDIANREDA);
    color_map.insert(Colors::Khaki1, KHAKI1);
    color_map.insert(Colors::Khaki3, KHAKI3);
    color_map.insert(Colors::LightCoral, LIGHTCORAL);
    color_map.insert(Colors::LightCyan1, LIGHTCYAN1);
    color_map.insert(Colors::LightCyan3, LIGHTCYAN3);
    color_map.insert(Colors::LightGoldenrod1, LIGHTGOLDENROD1);
    color_map.insert(Colors::LightGoldenrod2, LIGHTGOLDENROD2);
    color_map.insert(Colors::LightGoldenrod2a, LIGHTGOLDENROD2A);
    color_map.insert(Colors::LightGoldenrod2b, LIGHTGOLDENROD2B);
    color_map.insert(Colors::LightGoldenrod3, LIGHTGOLDENROD3);
    color_map.insert(Colors::LightGreen, LIGHTGREEN);
    color_map.insert(Colors::LightGreena, LIGHTGREENA);
    color_map.insert(Colors::LightPink1, LIGHTPINK1);
    color_map.insert(Colors::LightPink3, LIGHTPINK3);
    color_map.insert(Colors::LightPink4, LIGHTPINK4);
    color_map.insert(Colors::LightSalmon1, LIGHTSALMON1);
    color_map.insert(Colors::LightSalmon3, LIGHTSALMON3);
    color_map.insert(Colors::LightSalmon3a, LIGHTSALMON3A);
    color_map.insert(Colors::LightSeaGreen, LIGHTSEAGREEN);
    color_map.insert(Colors::LightSkyBlue1, LIGHTSKYBLUE1);
    color_map.insert(Colors::LightSkyBlue3, LIGHTSKYBLUE3);
    color_map.insert(Colors::LightSkyBlue3a, LIGHTSKYBLUE3A);
    color_map.insert(Colors::LightSlateBlue, LIGHTSLATEBLUE);
    color_map.insert(Colors::LightSlateGrey, LIGHTSLATEGREY);
    color_map.insert(Colors::LightSteelBlue1, LIGHTSTEELBLUE1);
    color_map.insert(Colors::LightSteelBlue3, LIGHTSTEELBLUE3);
    color_map.insert(Colors::LightSteelBlue, LIGHTSTEELBLUE);
    color_map.insert(Colors::LightYellow, LIGHTYELLOW);
    color_map.insert(Colors::Lime, LIME);
    color_map.insert(Colors::Magenta, MAGENTA);
    color_map.insert(Colors::Magenta1, MAGENTA1);
    color_map.insert(Colors::Magenta2, MAGENTA2);
    color_map.insert(Colors::Magenta3, MAGENTA3);
    color_map.insert(Colors::Magenta3a, MAGENTA3A);
    color_map.insert(Colors::Magenta3b, MAGENTA3B);
    color_map.insert(Colors::Maroon, MAROON);
    color_map.insert(Colors::MediumOrchid1, MEDIUMORCHID1);
    color_map.insert(Colors::MediumOrchid1a, MEDIUMORCHID1A);
    color_map.insert(Colors::MediumOrchid3, MEDIUMORCHID3);
    color_map.insert(Colors::MediumOrchid, MEDIUMORCHID);
    color_map.insert(Colors::MediumPurple1, MEDIUMPURPLE1);
    color_map.insert(Colors::MediumPurple2, MEDIUMPURPLE2);
    color_map.insert(Colors::MediumPurple2a, MEDIUMPURPLE2A);
    color_map.insert(Colors::MediumPurple3, MEDIUMPURPLE3);
    color_map.insert(Colors::MediumPurple3a, MEDIUMPURPLE3A);
    color_map.insert(Colors::MediumPurple4, MEDIUMPURPLE4);
    color_map.insert(Colors::MediumPurple, MEDIUMPURPLE);
    color_map.insert(Colors::MediumSpringGreen, MEDIUMSPRINGGREEN);
    color_map.insert(Colors::MediumTurquoise, MEDIUMTURQUOISE);
    color_map.insert(Colors::MediumVioletRed, MEDIUMVIOLETRED);
    color_map.insert(Colors::MistyRose1, MISTYROSE1);
    color_map.insert(Colors::MistyRose3, MISTYROSE3);
    color_map.insert(Colors::NavajoWhite1, NAVAJOWHITE1);
    color_map.insert(Colors::NavajoWhite3, NAVAJOWHITE3);
    color_map.insert(Colors::NavyBlue, NAVYBLUE);
    color_map.insert(Colors::Navy, NAVY);
    color_map.insert(Colors::Olive, OLIVE);
    color_map.insert(Colors::Orange, ORANGE);
    color_map.insert(Colors::Orange1, ORANGE1);
    color_map.insert(Colors::Orange2, ORANGE2);
    color_map.insert(Colors::Orange3, ORANGE3);
    color_map.insert(Colors::OrangeRed1, ORANGERED1);
    color_map.insert(Colors::Orchid1, ORCHID1);
    color_map.insert(Colors::Orchid2, ORCHID2);
    color_map.insert(Colors::Orchid, ORCHID);
    color_map.insert(Colors::PaleGreen1, PALEGREEN1);
    color_map.insert(Colors::PaleGreen1a, PALEGREEN1A);
    color_map.insert(Colors::PaleGreen3, PALEGREEN3);
    color_map.insert(Colors::PaleGreen3a, PALEGREEN3A);
    color_map.insert(Colors::PaleTurquoise1, PALETURQUOISE1);
    color_map.insert(Colors::PaleTurquoise4, PALETURQUOISE4);
    color_map.insert(Colors::PaleVioletRed1, PALEVIOLETRED1);
    color_map.insert(Colors::Pink1, PINK1);
    color_map.insert(Colors::Pink3, PINK3);
    color_map.insert(Colors::Plum1, PLUM1);
    color_map.insert(Colors::Plum2, PLUM2);
    color_map.insert(Colors::Plum3, PLUM3);
    color_map.insert(Colors::Plum4, PLUM4);
    color_map.insert(Colors::Purple3, PURPLE3);
    color_map.insert(Colors::Purple4, PURPLE4);
    color_map.insert(Colors::Purple4a, PURPLE4A);
    color_map.insert(Colors::Purple, PURPLE);
    color_map.insert(Colors::Purplea, PURPLEA);
    color_map.insert(Colors::Purpleb, PURPLEB);
    color_map.insert(Colors::Red1, RED1);
    color_map.insert(Colors::Red3, RED3);
    color_map.insert(Colors::Red3a, RED3A);
    color_map.insert(Colors::Red, RED);
    color_map.insert(Colors::RosyBrown, ROSYBROWN);
    color_map.insert(Colors::RoyalBlue1, ROYALBLUE1);
    color_map.insert(Colors::Salmon1, SALMON1);
    color_map.insert(Colors::SandyBrown, SANDYBROWN);
    color_map.insert(Colors::SeaGreen1, SEAGREEN1);
    color_map.insert(Colors::SeaGreen1a, SEAGREEN1A);
    color_map.insert(Colors::SeaGreen2, SEAGREEN2);
    color_map.insert(Colors::SeaGreen3, SEAGREEN3);
    color_map.insert(Colors::Silver, SILVER);
    color_map.insert(Colors::SkyBlue1, SKYBLUE1);
    color_map.insert(Colors::SkyBlue2, SKYBLUE2);
    color_map.insert(Colors::SkyBlue3, SKYBLUE3);
    color_map.insert(Colors::SlateBlue1, SLATEBLUE1);
    color_map.insert(Colors::SlateBlue3, SLATEBLUE3);
    color_map.insert(Colors::SlateBlue3a, SLATEBLUE3A);
    color_map.insert(Colors::SpringGreen1, SPRINGGREEN1);
    color_map.insert(Colors::SpringGreen2, SPRINGGREEN2);
    color_map.insert(Colors::SpringGreen2a, SPRINGGREEN2A);
    color_map.insert(Colors::SpringGreen3, SPRINGGREEN3);
    color_map.insert(Colors::SpringGreen3a, SPRINGGREEN3A);
    color_map.insert(Colors::SpringGreen4, SPRINGGREEN4);
    color_map.insert(Colors::SteelBlue1, STEELBLUE1);
    color_map.insert(Colors::SteelBlue1a, STEELBLUE1A);
    color_map.insert(Colors::SteelBlue3, STEELBLUE3);
    color_map.insert(Colors::SteelBlue, STEELBLUE);
    color_map.insert(Colors::Tan, TAN);
    color_map.insert(Colors::Teal, TEAL);
    color_map.insert(Colors::Thistle1, THISTLE1);
    color_map.insert(Colors::Thistle3, THISTLE3);
    color_map.insert(Colors::Turquoise2, TURQUOISE2);
    color_map.insert(Colors::Turquoise4, TURQUOISE4);
    color_map.insert(Colors::Violet, VIOLET);
    color_map.insert(Colors::Wheat1, WHEAT1);
    color_map.insert(Colors::Wheat4, WHEAT4);
    color_map.insert(Colors::White, WHITE);
    color_map.insert(Colors::Yellow1, YELLOW1);
    color_map.insert(Colors::Yellow2, YELLOW2);
    color_map.insert(Colors::Yellow3, YELLOW3);
    color_map.insert(Colors::Yellow3a, YELLOW3A);
    color_map.insert(Colors::Yellow4, YELLOW4);
    color_map.insert(Colors::Yellow4a, YELLOW4A);
    color_map.insert(Colors::Yellow, YELLOW);
    color_map
}
