extern crate neosys_native;
extern crate neocom_special;

use neocom_special::*;

struct Life {
  tick: u64,
}

impl Program for Life {
  fn new() -> Life {
    Life {
      tick: 0,
    }
  }

  fn title(&self) -> &str {
    "life"
  }

  fn update(&mut self, system: &mut System) {
    for _event in system.events() {
    }

    for pixel in system.display() {
      *pixel = Pixel {
        red:   255,
        green: 0,
        blue:  255,
      };
    }

    if self.tick == 1000 {
      system.quit();
    }

    self.tick += 1;
  }
}

fn main() {
  neosys_native::run::<Life>();
}
