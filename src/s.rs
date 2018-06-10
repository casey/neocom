pub const SURFACE_WIDTH:      usize = 512;
pub const SURFACE_HEIGHT:     usize = 512;
pub const SURFACE_PIXELS:     usize = SURFACE_WIDTH * SURFACE_HEIGHT;
pub const FRAMES_PER_SECOND:  usize = 60;

#[derive(Copy, Clone)]
pub struct Pixel {
  pub red:   u8,
  pub blue:  u8,
  pub green: u8,
}

pub struct Surface {
  pub pixels: Vec<Pixel>,
}

#[derive(Copy, Clone)]
pub enum Button {
  Left,
  Right,
  Up,
  Down,
  Action,
}

#[derive(Copy, Clone)]
pub enum Event {
  Down{button: Button},
  Up{button: Button},
}

pub struct Input {
  pub events: Vec<Event>,
}

pub struct Output {
  pub surface: Surface,
}

pub trait Game {
  fn new() -> Self where Self: Sized;
  fn frame(&mut self, input: Input) -> Output;
}

