
use winit::{ 
    event::{ElementState, Event, WindowEvent, KeyboardInput, VirtualKeyCode },
    event_loop::{ControlFlow, EventLoop },
    window::WindowBuilder
};

use isir::{ Renderer };

async fn run() {


    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();
    let size = (size.width, size.height);

    let mut renderer = Renderer::new(&window, size).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit
            }
            WindowEvent::Resized(size) => {
                let size = (size.width, size.height);
                renderer.resize(size);
                *control_flow = ControlFlow::Poll;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(code),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
            } => {
                if let VirtualKeyCode::Escape = code {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            match renderer.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                Err(wgpu::SurfaceError::OutOfMemory) =>
                    *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }

        _ => {}
    });

}


fn main() {
    pollster::block_on(run())
}
