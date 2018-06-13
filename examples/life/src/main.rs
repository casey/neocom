extern crate neosys_native;
extern crate neocom_special;
extern crate rand;

use rand::prelude::*;
use neocom_special::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
  Alive,
  Dead,
}

use self::Cell::*;

impl Cell {
  pub fn tick(self, neighbors: u8) -> Cell {
    match (self, neighbors) {
      (Alive, 0) => Dead,
      (Alive, 1) => Dead,
      (Alive, 2) => Alive,
      (Alive, 3) => Alive,
      (Alive, _) => Dead,
      (Dead,  0) => Dead,
      (Dead,  1) => Dead,
      (Dead,  2) => Dead,
      (Dead,  3) => Alive,
      (Dead,  _) => Dead,
    }
  }
}

struct Life {
  cells: Vec<Cell>,
}

impl Life {
  fn index(&self, x: usize, y: usize) -> usize {
    x + y * 512
  }

  fn neighbors(&self, i: usize) -> u8 {
    let mut neighbors = 0;

    let x = i % 512;
    let y = (i - x) / 512;

    let n = (y + 512 - 1) % 512;
    let e = (x               + 1) % 512;
    let s = (y               + 1) % 512;
    let w = (x + 512  - 1) % 512;

    for y in &[n, y, s] {
      for x in &[w, x, e] {
        let ni = self.index(*x, *y);
      
        if ni == i {
          continue;
        }

        if self.cells[ni] == Alive {
          neighbors += 1;
        }
      }
    }

    neighbors
  }

  pub fn step(&mut self) {
    let cells = self.cells.iter().enumerate().map(|(i, cell)| {
      cell.tick(self.neighbors(i))
    }).collect::<Vec<Cell>>();

    self.cells = cells;
  }
}

impl Program for Life {
  fn new() -> Life {
    Life {
      cells: (0..512*512).into_iter().map(|_| if random() {
        Alive
      } else {
        Dead
      }).collect(),
    }
  }

  fn title(&self) -> &str {
    "life"
  }

  fn update(&mut self, system: &mut System) {
    for event in system.events() {
      match event {
        Event::Character{character} => println!("char: {}", character),
        _ => {}
      }
    }

    for (pixel, cell) in system.pixels().iter_mut().zip(&self.cells) {
      *pixel = match cell {
        Alive => Pixel {
          red:   random(),
          green: 0,
          blue:  random(),
        },
        Dead => Pixel {
          red:   0,
          green: 0,
          blue:  0,
        },
      };
    }

    self.step();
  }
}

fn main() {
  neosys_native::run::<Life>();
}
