/*imports that we need, don't touch that*/
use crate::tiles;
use tui::style::Color;
/*                  */
pub static QUIT_KEY:char    ='k'; //currently A - Z keys are supported

pub static SHOW_TITLES:bool = true; //if you want titles or not

pub static MARGINS:u16=2;

/* customize your colors */
pub static USER_THEME:tiles::ColorScheme=tiles::ColorScheme{
    /* label colors */
    foreground_color1   :Color::Blue,
    background_color1   :Color::Cyan,
    /* value value */
    foreground_color2   :Color::Blue,
    background_color2   :Color::Cyan
};
/* define your ascii art, i should add something default btw  */
pub static ASCII_ART:&str="
";
