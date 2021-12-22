use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal;
use crossterm::execute;

use crate::tiles;
use crate::configuration;
pub fn init_menu()->Result<(),io::Error> {


    let mut stdout = io::stdout();
    execute!(stdout,terminal::Clear(terminal::ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    'mainloop:loop {
        terminal.draw(|f| {
            let row=Layout::default()
                .direction(Direction::Horizontal)
                .margin(configuration::MARGINS)
                .constraints(
                    [
                        Constraint::Percentage(60),
                        Constraint::Percentage(40),
                        Constraint::Percentage(0)
                    ].as_ref()
                )
                .split(f.size());

            f.render_widget(tiles::text_tile(),row[1]);
            f.render_widget(tiles::ascii_tile(),row[0]);
        });
        let event = read()?;
        if event == Event::Key(KeyCode::Char(configuration::QUIT_KEY).into()) {
            break 'mainloop;
        }
       
       
    }
    Ok(())
    
}
