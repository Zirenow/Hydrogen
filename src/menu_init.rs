use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block,Borders};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use crossterm::terminal::Clear;
use crossterm::execute;

pub fn init_menu()->Result<(),io::Error> {


    let mut stdout = io::stdout();
    execute!(stdout,terminal::Clear(terminal::ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app_name="Hydrogen-indev";
    let exit_key:char='k';

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(app_name)
            .borders(Borders::ALL);
        f.render_widget(block, size);
    });

    loop {
        let event = read()?;
        if event == Event::Key(KeyCode::Char(exit_key).into()) {
            break;
        }
    }
    Ok(())
}
