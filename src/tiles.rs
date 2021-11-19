use tui::widgets::{Wrap,Paragraph,Block,Borders, Sparkline, Chart,Dataset, Axis, GraphType};
use tui::layout::Alignment;
use tui::style::{Style,Color, Modifier};
use tui::text::{Spans, Span};
use tui::symbols;
use crate::configuration;
use sysinfo::{ProcessorExt,System, SystemExt, Disk};

static MAGABYTE:u64=1024;
pub fn text_tile()->Paragraph<'static>{
    let mut sys=System::new_all();

    let blue_style=Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD);
    let magenta_style=Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD);
    let cyan_style=Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let light_magenta_style=Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    let usage= sys.used_memory();
    let total=sys.total_memory();
    sys.refresh_system();
    let percent=usage/total;
    println!("{}",percent);
    let os_block=vec![
        Spans::from(vec![
            Span::styled("OS type: ",blue_style),
            Span::styled(format!("{}",sys.name().unwrap()),cyan_style),
        ]),
        Spans::from(vec![
            Span::styled("Host name: ",blue_style),
            Span::styled(format!("{}",sys.host_name().unwrap()),cyan_style),
        ]),
        Spans::from(vec![
            Span::styled("Kernel version: ",blue_style),
            Span::styled(format!("{}",sys.kernel_version().unwrap()),cyan_style),
        ]),
        Spans::from(vec![
            Span::styled("Uptime: ",blue_style),
            Span::styled(format!("{} minutes",sys.uptime()/60),cyan_style),
        ]),
        Spans::from(vec![
            Span::styled("",magenta_style),
        ]),

        Spans::from(vec![
            Span::styled("Total Ram: ",magenta_style),
            Span::styled(format!("{} Mb",sys.total_memory() /MAGABYTE),light_magenta_style),
        ]),
        Spans::from(vec![
            Span::styled("Free Ram: ",magenta_style),
            Span::styled(format!("{} Mb",sys.free_memory() /MAGABYTE),light_magenta_style),
        ]),
        Spans::from(vec![
            Span::styled("Used Ram: ",magenta_style),
            Span::styled(format!("{} Mb",sys.used_memory() /MAGABYTE),light_magenta_style),
        ]),
        Spans::from(vec![
            Span::styled("SWAP: ",magenta_style),
            Span::styled(format!("{} Mb",sys.total_swap() /MAGABYTE),light_magenta_style),
        ]),
        Spans::from(vec![
            Span::styled("Used SWAP: ",magenta_style),
            Span::styled(format!("{} Mb",sys.used_swap() /MAGABYTE),light_magenta_style)
        ]),
        Spans::from(vec![
            Span::styled("",magenta_style),
        ]),
        Spans::from(vec![
            Span::styled("Logical CPU's: ",magenta_style),
            Span::styled(format!("{}",sys.processors().len()),magenta_style)
        ]),
        Spans::from(vec![
            Span::styled("Global CPU usage: ",magenta_style),
            Span::styled(format!("{:.2} %",sys.global_processor_info().cpu_usage()),magenta_style)
        ]),
        Spans::from(vec![
            Span::styled("Number of cores: ",magenta_style),
            Span::styled(format!("{:?}",sys.physical_core_count().unwrap()),magenta_style)
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

    let tile=Paragraph::new(configuration::ASCII_ART)
        .block(Block::default()
            .title_alignment(Alignment::Center)

            .borders(Borders::ALL)) .alignment(Alignment::Center);
    return tile;
}
