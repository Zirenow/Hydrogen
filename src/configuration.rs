/*imports that we need, don't touch that*/
use crate::tiles;
use tui::style::Color;

/*                  */
pub static QUIT_KEY:char    ='k'; //currently A - Z keys are supported

pub static SHOW_TITLES:bool = false; //if you want titles or not

pub static SHOW_BORDERS:bool = true;  //if you want borders or not

/* 
It's an enum with two states: Normal and Extended
Use Mode::Extended if you want to have process list instead of Ascii logo
*/
pub static APPLICATION_MODE:tiles::Mode = tiles::Mode::Extended;

pub static EXIT_AFTER_EXECUTION:bool    = true; // program will exit just after printing information

pub static MARGINS:u16=1;

/* customize your colors (visit https://docs.rs/tui/0.16.0/tui/style/enum.Color.html if you need help) */
pub static USER_THEME:tiles::ColorScheme=tiles::ColorScheme{
    /* label colors */
    foreground_color1   :Color::Blue,
    background_color1   :Color::Black,
    /* value value */
    foreground_color2   :Color::Cyan,
    background_color2   :Color::Black
};


/* define your ascii art if you want  */
pub static ASCII_ART:&str="\n\n\n\n\n
 _    _           _                            
| |  | |         | |                           
| |__| |_   _  __| |_ __ ___   __ _  ___ _ __  
|  __  | | | |/ _` | '__/ _ \\ / _` |/ _ \\ '_  \\
| |  | | |_| | (_| | | | (_) | (_| |  __/ | | |
|_|  |_|\\__, |\\__,_|_|  \\___/ \\__, |\\ __|_| |_|
         __/ |   _ __ ___      __/ |           
        |___/   | '__/ __|    |___/            
               _| |  \\__ \\                    
              (_)_|  |___/                     
                                               
                                             
                                            
";
                                                                            

