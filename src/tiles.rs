use tui::widgets::{Wrap,Paragraph,Block,Borders, Sparkline, Chart,Dataset, Axis, GraphType};
use tui::layout::Alignment;
use tui::style::{Style,Color, Modifier};
use tui::text::{Spans, Span};
use tui::symbols;
use crate::configuration;
use sysinfo::{ProcessorExt,System, SystemExt, Disk};

static MAGABYTE:u64=1024;
pub struct ColorScheme{
    pub foreground_color1:Color,
    pub background_color1:Color,
    pub foreground_color2:Color,
    pub background_color2:Color
}
pub fn text_tile()->Paragraph<'static>{
    let mut sys=System::new_all();

    let label_style=Style::default()
        .fg(configuration::USER_THEME.foreground_color1)
        .bg(configuration::USER_THEME.background_color1);
    let value_style=Style::default()
        .fg(configuration::USER_THEME.foreground_color2)
        .bg(configuration::USER_THEME.background_color2);

    let usage= sys.used_memory();
    let total=sys.total_memory();
    sys.refresh_system();


    let os_block=vec![
        Spans::from(vec![
            Span::styled("OS type: ",label_style),
            Span::styled(format!("{}",sys.name().unwrap()),value_style),
        ]),
        Spans::from(vec![
            Span::styled("Host name: ",label_style),
            Span::styled(format!("{}",sys.host_name().unwrap()),value_style),
        ]),
        Spans::from(vec![
            Span::styled("Kernel version: ",label_style),
            Span::styled(format!("{}",sys.kernel_version().unwrap()),value_style),
        ]),
        Spans::from(vec![
            Span::styled("Uptime: ",label_style),
            Span::styled(format!("{} minutes",sys.uptime()/60),value_style),
        ]),
        Spans::from(vec![
            Span::styled("",label_style),
        ]),

        Spans::from(vec![
            Span::styled("Total Ram: ",label_style),
            Span::styled(format!("{} Mb",sys.total_memory() /MAGABYTE),value_style),
        ]),
        Spans::from(vec![
            Span::styled("Free Ram: ",label_style),
            Span::styled(format!("{} Mb",sys.free_memory() /MAGABYTE),value_style),
        ]),
        Spans::from(vec![
            Span::styled("Used Ram: ",label_style),
            Span::styled(format!("{} Mb",sys.used_memory() /MAGABYTE),value_style),
        ]),
        Spans::from(vec![
            Span::styled("SWAP: ",label_style),
            Span::styled(format!("{} Mb",sys.total_swap() /MAGABYTE),value_style),
        ]),
        Spans::from(vec![
            Span::styled("Used SWAP: ",label_style),
            Span::styled(format!("{} Mb",sys.used_swap() /MAGABYTE),value_style)
        ]),
        Spans::from(vec![
            Span::styled("",label_style),
        ]),
        Spans::from(vec![
            Span::styled("Logical CPU's: ",label_style),
            Span::styled(format!("{}",sys.processors().len()),value_style)
        ]),
        Spans::from(vec![
            Span::styled("Global CPU usage: ",label_style),
            Span::styled(format!("{:.2} %",sys.global_processor_info().cpu_usage()),value_style)
        ]),
        Spans::from(vec![
            Span::styled("Number of cores: ",label_style),
            Span::styled(format!("{:?}",sys.physical_core_count().unwrap()),value_style)
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
