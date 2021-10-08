use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block,Borders};
use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;
pub fn init_menu()->Result<(),io::Error> {
    //println!("its working");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let exit_key:char='k';
    loop {
        let event = read()?;
        crossterm::terminal::enable_raw_mode();

        if event == Event::Key(KeyCode::Char(exit_key).into()) {
            break;
        }
    }
    Ok(())
}