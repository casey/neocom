extern crate cpal;
extern crate gl;
extern crate glutin;
extern crate neocom_special;

mod misc;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::sync::{Arc, Mutex};

use neocom_special::*;

use misc::*;

struct Native {
    quit: bool,
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

use glutin::GlContext;

pub fn run<P: Program + 'static>() {
    let program = Arc::new(Mutex::new(P::new()));
    run_audio::<P>(Arc::clone(&program));
    run_graphics::<P>(Arc::clone(&program));
}

fn run_audio<P: Program + 'static>(program: Arc<Mutex<P>>) {
    use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};
    extern crate rand;
    std::thread::spawn(move || {
        let event_loop = EventLoop::new();
        let device = cpal::default_output_device().expect("no output device available");
        let mut supported_formats_range = device
            .supported_output_formats()
            .expect("error while querying formats");
        let format = supported_formats_range
            .next()
            .expect("no supported format?!")
            .with_max_sample_rate();
        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
        event_loop.play_stream(stream_id);
        event_loop.run(move |_stream_id, stream_data| match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                let p: &P = &*program.lock().unwrap();
                P::sound(p, buffer.iter_mut());
            }
            _ => {
                println!("not");
            }
        });
    });
}

fn run_graphics<P: Program>(program: Arc<Mutex<P>>) {
    let mut system = Native {
        quit: false,
        events: vec![],
        pixels: vec![
            Pixel {
                red: 0,
                green: 255,
                blue: 0
            };
            SURFACE_PIXELS as usize
        ],
    };

    let mut events_loop = glutin::EventsLoop::new();

    let window = {
        let p = program.lock().unwrap();
        glutin::WindowBuilder::new()
            .with_title(p.title())
            .with_dimensions(SURFACE_WIDTH, SURFACE_HEIGHT)
    };

    let context = glutin::ContextBuilder::new().with_vsync(true);

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

        let pixel_uniform =
            gl::GetUniformLocation(gl_program, CString::new("pixels").unwrap().as_ptr());
        gl::Uniform1i(pixel_uniform, 0);
        gl::ActiveTexture(gl::TEXTURE0 + 0);

        // Specify the layout of the vertex data
        let pos_attr =
            gl::GetAttribLocation(gl_program, CString::new("position").unwrap().as_ptr());
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
                glutin::Event::WindowEvent { event, .. } => match event {
                    CloseRequested => system.quit = true,
                    Resized(w, h) => gl_window.resize(w, h),
                    ReceivedCharacter(character) => {
                        system.events.push(Event::Character { character })
                    }
                    _ => (),
                },
                _ => (),
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        {
            let mut p = program.lock().unwrap();
            p.update(&mut system);
        }

        let pixels = system.pixels.as_ptr();
        let bytes = pixels as *const std::os::raw::c_void;

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                512,
                512,
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
