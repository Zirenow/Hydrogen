use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Wrap,Paragraph,Block,Borders};
use tui::layout::{Layout, Constraint, Direction};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use crossterm::terminal::Clear;
use crossterm::execute;

use crate::tiles;
use crate::configuration;
pub fn init_menu()->Result<(),io::Error> {


    let mut stdout = io::stdout();
    execute!(stdout,terminal::Clear(terminal::ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app_name="Hydrogen-indev";
    let exit_key:char='k';

    terminal.draw(|f| {
        let row=Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Percentage(0)
                ].as_ref()
            )
            .split(f.size());

        let block = Block::default()
            .title("Block 1")
            .borders(Borders::ALL);
        let block2=Block::default()
            .title("Block 2")
            .borders(Borders::ALL);
        let content="Lorem ipsum dolor\n sit amet";
        let text=Paragraph::new(content)
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            //.alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(block, row[0]);
        f.render_widget(tiles::os_tile(),row[1]);

    });

    loop {
        let event = read()?;
        if event == Event::Key(KeyCode::Char(configuration::QUIT_KEY).into()) {
            break;
        }
    }
    Ok(())
}
