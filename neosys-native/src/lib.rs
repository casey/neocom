extern crate neocom_special;

use neocom_special::*;

struct Native {
  quit:    bool,
  display: Vec<Pixel>
}

impl System for Native {
  fn events(&self) -> &[Event] {
    &[]
  }

  fn display(&mut self) -> &mut [Pixel] {
    &mut self.display
  }

  fn quit(&mut self) {
    self.quit = true;
  }
}

extern crate gl;
extern crate glutin;

use glutin::{ControlFlow, GlContext};

pub fn run<P: Program>() {
  let mut system  = Native {
    quit:    false,
    display: vec![Pixel{red: 0, green: 255, blue: 0}; SURFACE_PIXELS as usize],
  };

  let mut program = P::new();

  let mut events_loop = glutin::EventsLoop::new();

  let window = glutin::WindowBuilder::new()
    .with_title(program.title())
    .with_dimensions(SURFACE_WIDTH, SURFACE_HEIGHT);

  let context = glutin::ContextBuilder::new()
    .with_vsync(true);

  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
  }

  events_loop.run_forever(|event| {
    match event {
      glutin::Event::WindowEvent{ event, .. } => match event {
        glutin::WindowEvent::CloseRequested => system.quit = true,
        glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
        _ => ()
      },
      _ => ()
    }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    program.update(&mut system);

    gl_window.swap_buffers().unwrap();

    if system.quit {
      ControlFlow::Break
    } else {
      ControlFlow::Continue
    }
  })
}
