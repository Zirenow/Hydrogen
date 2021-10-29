use tui::widgets::{Wrap,Paragraph,Block,Borders};
use crate::configuration;



pub fn os_tile()->Paragraph<'static>{

    let contents=format!(
        "OS type: {}\n
         Kernel version:\n
         Uptime: {}",
         sys_info::os_type().unwrap(),
         sys_info::boottime().unwrap());
    let title="OS";


    if configuration::SHOW_TITLES==true{
        let tile=Paragraph::new(contents)
            .block(Block::default()
                .title(title)
                .borders(Borders::ALL))
            //.alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
            return tile;
    }
    else {
        let tile=Paragraph::new(contents)
        .block(Block::default()
            .borders(Borders::ALL))
        .wrap(Wrap { trim: true });
        return tile; }

}