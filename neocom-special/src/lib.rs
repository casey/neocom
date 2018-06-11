pub const SURFACE_WIDTH:      u32 = 512;
pub const SURFACE_HEIGHT:     u32 = 512;
pub const SURFACE_PIXELS:     u32 = SURFACE_WIDTH * SURFACE_HEIGHT;
pub const FRAMES_PER_SECOND:  u32 = 60;

#[derive(Copy, Clone)]
pub struct Pixel {
  pub red:   u8,
  pub blue:  u8,
  pub green: u8,
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

pub trait System {
  fn events(&self) -> &[Event];
  fn display(&mut self) -> &mut [Pixel];
  fn quit(&mut self);
}

pub trait Program {
  fn new() -> Self where Self: Sized;
  fn update(&mut self, system: &mut System);
  fn title(&self) -> &str {
    "ネオ・SPECIAL"
  }
}
