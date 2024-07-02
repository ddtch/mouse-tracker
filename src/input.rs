use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

pub struct InputTracker {
    pub left_clicks: i32,
    pub right_clicks: i32,
    pub movement: f64,
    pub scrolls: i32,
}

impl InputTracker {
    pub fn new() -> Self {
        InputTracker {
            left_clicks: 0,
            right_clicks: 0,
            movement: 0.0,
            scrolls: 0,
        }
    }

    pub fn track(&mut self, event_loop: &mut EventLoop<()>) {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    self.movement += (delta.0.powi(2) + delta.1.powi(2)).sqrt();
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    ..
                } => match (state, button) {
                    (ElementState::Pressed, winit::event::MouseButton::Left) => {
                        self.left_clicks += 1;
                    }
                    (ElementState::Pressed, winit::event::MouseButton::Right) => {
                        self.right_clicks += 1;
                    }
                    _ => (),
                },
                Event::WindowEvent {
                    event: WindowEvent::MouseWheel { delta, .. },
                    ..
                } => match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.scrolls += y as i32;
                    }
                    MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                        self.scrolls += y as i32;
                    }
                },
                Event::MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            }
        });
    }
}
