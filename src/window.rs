pub struct SDLWindow {
    pub context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
}

impl SDLWindow {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("gemuboy", 160, 144)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();
        canvas.set_logical_size(144, 160).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        SDLWindow {
            context: sdl_context,
            video_subsystem: video_subsystem,
            canvas: canvas,
            event_pump: event_pump,
        }
    }
}