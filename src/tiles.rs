use tui::widgets::{Wrap,Paragraph,Block,Borders};
use tui::layout::Alignment;
use tui::style::{Style,Color};
use tui::text::{Spans, Span};
use crate::configuration;
use sysinfo::{ProcessExt,ProcessorExt,System, SystemExt};
use std::collections::HashMap;
static MAGABYTE:u64=1024;
pub struct ColorScheme{
    pub foreground_color1:Color,
    pub background_color1:Color,
    pub foreground_color2:Color,
    pub background_color2:Color
}


impl ColorScheme{
    pub fn as_label_style(&self)->Style{
        Style::default()
            .fg(self.foreground_color1)
            .bg(self.background_color1)
    }
    pub fn as_value_style(&self)->Style{
        Style::default()
            .fg(self.foreground_color2)
            .bg(self.background_color2)
    }
}

pub fn text_tile(sys:&System)->Paragraph<'static>{
    //let mut sys=System::new_all();

    let label_style=configuration::USER_THEME.as_label_style();
        
    let value_style=configuration::USER_THEME.as_value_style();

    
    
   
    //sys.refresh_system();
    

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
            Span::styled("CPU: ",label_style),
            Span::styled(format!("{}",sys.global_processor_info().brand()),value_style)
        ]),
        Spans::from(vec![
            Span::styled("Logical CPU's: ",label_style),
            Span::styled(format!("{}",sys.processors().len()),value_style)
        ]),
        Spans::from(vec![
            Span::styled("CPU usage: ",label_style),
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
                .borders(if configuration::SHOW_BORDERS==true{Borders::ALL} 
                         else{Borders::NONE}))
            .wrap(Wrap { trim: true });
            return tile;
    }
    else {
        let tile=Paragraph::new(os_block)
        .block(Block::default()
            .borders(if configuration::SHOW_BORDERS==true{Borders::ALL}
                     else{Borders::NONE}))

        .wrap(Wrap { trim: true });
        return tile; }

}

pub fn ascii_tile()->Paragraph<'static>{

    let tile=Paragraph::new(configuration::ASCII_ART)
        .block(Block::default()
            .title_alignment(Alignment::Center)
           
            .borders(if configuration::SHOW_BORDERS==true{Borders::ALL}
                     else{Borders::NONE}))
                .alignment(Alignment::Center);
                
        
    return tile;
}


pub fn proc_tile(base:&System)->Paragraph<'static>{
   
    let mut data:Vec<Spans> = Vec::new();

    for (pid, proc) in base.processes(){
        data.push(Spans::from(vec![
            Span::styled("PID:",                    configuration::USER_THEME.as_label_style()),
            Span::styled(format!("{} ",pid),        configuration::USER_THEME.as_value_style()),
            Span::styled("Name: ",                  configuration::USER_THEME.as_label_style()),
            Span::styled(format!("{}",proc.name()), configuration::USER_THEME.as_value_style())
        ]));
        
    }
    Paragraph::new(data)
        .block(Block::default()
        .borders(if configuration::SHOW_BORDERS==true{Borders::ALL}
                 else{Borders::NONE}))
        .alignment(Alignment::Left)
        
}
