use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String>{

    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let window = video_subsystem.window("Rustgine", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string());


        let mut event_pump = sdl.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter(){
                match event {
                    Event::Quit { .. } | Event::KeyDown{ keycode: Some(Keycode::Escape), .. } => {break 'running}
                    _ => {}
                }
            }
        }

    Ok(())
}