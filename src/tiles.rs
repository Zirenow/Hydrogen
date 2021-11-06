use tui::widgets::{Wrap,Paragraph,Block,Borders, Sparkline, Chart,Dataset, Axis, GraphType};
use tui::layout::Alignment;
use tui::style::{Style,Color, Modifier};
use tui::text::{Spans, Span};
use tui::symbols;
use crate::configuration;
use sysinfo::{System, SystemExt, Disk};

static MAGABYTE:u64=1024;
pub fn text_tile()->Paragraph<'static>{
    let mut sys=System::new_all();

    let blue_style=Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let orange_style=Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD);
    let usage= (sys.used_memory() /sys.total_memory()) as f64;
    let os_block=vec![
        Spans::from(vec![
            Span::styled("OS type: ",blue_style),
            Span::raw(format!("{}",sys.name().unwrap())),
        ]),
        Spans::from(vec![
            Span::styled("Host name:: ",blue_style),
            Span::raw(format!("{}",sys.host_name().unwrap())),
        ]),
        Spans::from(vec![
            Span::styled("Kernel version: ",blue_style),
            Span::raw(format!("{}",sys.kernel_version().unwrap())),
        ]),
        Spans::from(vec![
            Span::styled("Uptime: ",blue_style),
            Span::raw(format!("{} minutes",sys.uptime()/60)),
        ]),
        Spans::from(vec![
            Span::styled("",orange_style),
        ]),

        Spans::from(vec![
            Span::styled("Total Ram: ",orange_style),
            Span::raw(format!("{} Mb",sys.total_memory() /MAGABYTE)),
        ]),
        Spans::from(vec![
            Span::styled("Free Ram: ",orange_style),
            Span::raw(format!("{} Mb",sys.free_memory() /MAGABYTE)),
        ]),
        Spans::from(vec![
            Span::styled("Used Ram: ",orange_style),
            Span::raw(format!("{} Mb",sys.used_memory() /MAGABYTE)),
        ]),
        Spans::from(vec![
            Span::styled("Used Ram %: ",orange_style),
            Span::raw(format!("{} %",usage)),
        ]),
        Spans::from(vec![
            Span::styled("SWAP: ",orange_style),
            Span::raw(format!("{} Mb",sys.total_swap() /MAGABYTE)),
        ]),
        Spans::from(vec![
            Span::styled("Used SWAP: ",orange_style),
            Span::raw(format!("{} Mb",sys.used_swap() /MAGABYTE)),
        ]),

    ];






    let title="Hydrogen";


    if configuration::SHOW_TITLES==true{
        let tile=Paragraph::new(os_block)
            .block(Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL))
            //.alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
            return tile;
    }
    else {
        let tile=Paragraph::new(os_block)
        .block(Block::default()
            .borders(Borders::ALL))

        .wrap(Wrap { trim: true });
        return tile; }

}
pub fn ascii_tile()->Paragraph<'static>{
    let text =
    "\n\n\n\n\n
    ,-~¨-``-¨-,           _,
    /          / ;^-._...,¨/
    /          / /         /
    /          / /         /
    /          / /         /
    /,.-:''-,_ / /         /
    _,.-:--._  ^ ^:-._ __../
    /^         / /¨:.._¨__.;
    /          / /      ^  /
    /          / /         /
    /          / /         /
    /_,.--:^-._/ /         /
    ^           ^¨¨-.___.:^";


    let tile=Paragraph::new(text)
        .block(Block::default()
            .title_alignment(Alignment::Center)

            .borders(Borders::ALL)) .alignment(Alignment::Center);
    return tile;
}
