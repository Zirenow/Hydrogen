use std::io;
use std::{thread,time};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal;
use crossterm::execute;
use sysinfo::{ProcessExt,ProcessorExt,System, SystemExt};
use crate::tiles;
use crate::configuration;
pub fn init_menu()->Result<(),io::Error> {


    let mut stdout = io::stdout();
    execute!(stdout,terminal::Clear(terminal::ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    //let mut data=System::new_all();
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

            let mut data=System::new_all(); /* Yes, this is bad for memory but it's working */

            f.render_widget(tiles::text_tile(&data),row[1]);
            f.render_widget(tiles::proc_tile(&data),row[0]);

            data.refresh_system();
            

            thread::sleep(time::Duration::from_secs(1));


        });
        
    }
    Ok(())
    
}
