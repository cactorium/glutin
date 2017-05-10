extern crate glutin;

mod support;

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("glutin - Cursor grabbing test")
        .build(&events_loop)
        .unwrap();

    let _ = unsafe { window.make_current() };

    let context = support::load(&window);
    let mut grabbed = false;

    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {

                glutin::WindowEvent::KeyboardInput(glutin::ElementState::Pressed, _, _, _) => {
                    if grabbed {
                        grabbed = false;
                        window.set_cursor_state(glutin::CursorState::Normal)
                              .ok().expect("could not ungrab mouse cursor");
                    } else {
                        grabbed = true;
                        window.set_cursor_state(glutin::CursorState::Grab)
                              .ok().expect("could not grab mouse cursor");
                    }
                },

                glutin::WindowEvent::Closed => events_loop.interrupt(),

                a @ glutin::WindowEvent::MouseMoved(_, _) => {
                    println!("{:?}", a);
                },

                _ => (),
            },
        }

        context.draw_frame((0.0, 1.0, 0.0, 1.0));
        let _ = window.swap_buffers();
    });
}
