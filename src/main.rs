mod camera;
mod model;

use std::{
    f32::consts::PI,
    io::{stdout, Write},
};

use camera::Camera;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::EnterAlternateScreen,
    QueueableCommand, Result,
};
use model::{Scene, Screen};

fn main() -> Result<()> {
    let size = crossterm::terminal::size().unwrap_or((90, 30));
    let original_y = -00.;
    let mut cam = Camera::new(
        0.,
        original_y,
        30.,
        PI / 3.,
        PI / 3. * ((size.1 as f32) / (size.0 as f32)) * 2.2,
    );
    let mut screen = Screen::new((size.0) as usize, (size.1) as usize);
    let scene = Scene::new(2.5, 2., 0.66, 12., 4., 0., 0.);
    execute!(stdout(), EnterAlternateScreen,)?;
    let mut i = 0.;
    loop {
        let scene_ti = scene.at_t(i);
        screen.clear();
        cam.fill(&mut screen, &scene_ti);
        draw(&screen)?;
        i += 1.;
        cam.position.y = original_y + (scene_ti.ge * 1.3).cos() * 10.;
    }
}

fn draw(screen: &Screen) -> Result<()> {
    for i in 0..screen.height {
        stdout().queue(MoveTo(0, i as u16))?;
        let width = screen
            .buffer
            .get(i)
            .expect("You try to read content outside of the screen height!");
        for (r, g, b) in width {
            stdout()
                .queue(SetBackgroundColor(Color::Rgb {
                    r: *r,
                    g: *g,
                    b: *b,
                }))?
                .queue(SetForegroundColor(Color::Rgb {
                    r: *r,
                    g: *g,
                    b: *b,
                }))?
                .queue(Print("."))?;
        }
    }
    stdout().queue(MoveTo(1, 1))?;
    stdout().flush()?;
    Ok(())
}
