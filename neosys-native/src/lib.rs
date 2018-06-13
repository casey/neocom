extern crate neocom_special;
extern crate gl;
extern crate glutin;

mod misc;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use neocom_special::*;

use misc::*;

struct Native {
  quit:   bool,
  events: Vec<Event>,
  pixels: Vec<Pixel>,
}

impl System for Native {
  fn events(&self) -> &[Event] {
    &self.events
  }

  fn pixels(&mut self) -> &mut [Pixel] {
    &mut self.pixels
  }

  fn quit(&mut self) {
    self.quit = true;
  }
}

use glutin:: GlContext;

pub fn run<P: Program>() {
  let mut system  = Native {
    quit:   false,
    events: vec![],
    pixels: vec![Pixel{red: 0, green: 255, blue: 0}; SURFACE_PIXELS as usize],
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

  let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
  let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
  let gl_program = link_program(vs, fs);
  let mut vao = 0;
  let mut vbo = 0;
  let mut texture = 0;

  unsafe {
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

    // Create a Vertex Buffer Object and copy the vertex data to it
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        mem::transmute(&VERTEX_DATA[0]),
        gl::STATIC_DRAW,
    );

    // Use shader program
    gl::UseProgram(gl_program);
    gl::BindFragDataLocation(gl_program, 0, CString::new("color").unwrap().as_ptr());

    let pixel_uniform = gl::GetUniformLocation(gl_program, CString::new("pixels").unwrap().as_ptr());
    gl::Uniform1i(pixel_uniform, 0);
    gl::ActiveTexture(gl::TEXTURE0 + 0);

    // Specify the layout of the vertex data
    let pos_attr = gl::GetAttribLocation(gl_program, CString::new("position").unwrap().as_ptr());
    gl::EnableVertexAttribArray(pos_attr as GLuint);
    gl::VertexAttribPointer(
        pos_attr as GLuint,
        4,
        gl::FLOAT,
        gl::FALSE as GLboolean,
        0,
        ptr::null(),
    );
  }

  while !system.quit {
    system.events.clear();

    events_loop.poll_events(|event| {
      use glutin::WindowEvent::*;
      match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
          CloseRequested               => system.quit = true,
          Resized(w, h)                => gl_window.resize(w, h),
          ReceivedCharacter(character) => system.events.push(Event::Character{character}),
          _ => ()
        },
        _ => ()
      }
    });

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    program.update(&mut system);

    let pixels = system.pixels.as_ptr();
    let bytes = pixels as *const std::os::raw::c_void;

    unsafe {
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        512, 512,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        bytes,
      );

      gl::DrawArrays(gl::TRIANGLES, 0, 6);

      /*
      let error = gl::GetError();

      if error != gl::NO_ERROR {
        panic!("{}", error);
      }
      */
    }

    gl_window.swap_buffers().unwrap();
  }

  unsafe {
      gl::DeleteProgram(gl_program);
      gl::DeleteShader(fs);
      gl::DeleteShader(vs);
      gl::DeleteBuffers(1, &vbo);
      gl::DeleteVertexArrays(1, &vao);
  }
}
