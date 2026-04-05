use ratatui::{
    DefaultTerminal, Frame,
    text::Line,
    widgets::{Block, Borders},
};
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let b = Block::default()
        .borders(Borders::ALL)
        .title(Line::from("Tetris").centered());
    frame.render_widget(b, frame.area());
}
