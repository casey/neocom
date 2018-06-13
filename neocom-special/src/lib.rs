pub const SURFACE_WIDTH:      u32 = 512;
pub const SURFACE_HEIGHT:     u32 = 512;
pub const SURFACE_PIXELS:     u32 = SURFACE_WIDTH * SURFACE_HEIGHT;
pub const FRAMES_PER_SECOND:  u32 = 60;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
  pub red:   u8,
  pub green: u8,
  pub blue:  u8,
}

#[derive(Copy, Clone, Debug)]
pub enum Button {
  Left,
  Right,
  Up,
  Down,
  Action,
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
  Press{button: Button},
  Release{button: Button},
  Character{character: char},
}

pub trait System {
  fn events(&self) -> &[Event];
  fn pixels(&mut self) -> &mut [Pixel];
  fn quit(&mut self);
}

pub trait Program {
  fn new() -> Self;
  fn update(&mut self, system: &mut System);
  fn sound(&mut [f32]) {}
  fn title(&self) -> &str {
    "ネオ・SPECIAL"
  }
}
