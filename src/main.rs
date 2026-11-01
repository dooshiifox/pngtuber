mod log;

use std::{num::NonZeroU32, sync::Arc};

use softbuffer::{Context, Surface};
use tracing::{error, info};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::ModifiersState,
    platform::startup_notify::{EventLoopExtStartupNotify, WindowAttributesExtStartupNotify},
    raw_window_handle::{DisplayHandle, HasDisplayHandle},
    window::{CursorGrabMode, Window},
};

fn main() {
    crate::log::init_logging(None);

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    let _event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        info!("Starting to send user event every second");
        loop {
            let _ = _event_loop_proxy.send_event(UserEvent::WakeUp);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut state = Application::new(&event_loop);
    event_loop.run_app(&mut state).unwrap();
}

struct Application {
    window: Option<WindowState>,
    context: Option<Context<DisplayHandle<'static>>>,
}
impl Application {
    fn new<T>(event_loop: &EventLoop<T>) -> Self {
        // SAFETY: we drop the context right before the event loop is stopped, thus making it safe.
        let context = Some(
            Context::new(unsafe {
                std::mem::transmute::<DisplayHandle<'_>, DisplayHandle<'static>>(
                    event_loop.display_handle().unwrap(),
                )
            })
            .unwrap(),
        );

        Self {
            context,
            window: None,
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        info!("creating a window!");

        let mut attrs = Window::default_attributes()
            .with_title("Hello World")
            .with_transparent(true)
            .with_decorations(false)
            .with_blur(true);
        if let Some(token) = event_loop.read_token_from_env() {
            winit::platform::startup_notify::reset_activation_token_env();
            info!("Using token {:?} to activate a window", token);
            attrs = attrs.with_activation_token(token);
        }

        let window = event_loop.create_window(attrs).unwrap();
        let state = WindowState::new(self, window);
        self.window = Some(state);
    }
}
impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent as WE;
        let Some(window) = &mut self.window else {
            return;
        };

        match event {
            WE::CloseRequested => {
                self.window = None;
            }
            WE::Resized(size) => window.resize(size),
            WE::RedrawRequested => {
                if let Err(err) = window.draw() {
                    error!("Error drawing window: {err}");
                }
            }
            WE::ActivationTokenDone { token: _token, .. } => {
                winit::platform::startup_notify::set_activation_token_env(_token);
            }
            _ => {
                info!("window event received: {event:?}");
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        info!("Device {device_id:?} event: {event:?}");
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        info!("User event: {event:?}");
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            info!("No windows left, exiting...");
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // We must drop the context here.
        self.context = None;
    }
}

struct WindowState {
    surface: Surface<DisplayHandle<'static>, Arc<Window>>,
    window: Arc<Window>,
    cursor_position: Option<PhysicalPosition<f64>>,
    modifiers: ModifiersState,
    cursor_grab: CursorGrabMode,
    zoom: f64,
    rotated: f32,
    panned: PhysicalPosition<f32>,
}
impl WindowState {
    fn new(app: &Application, window: Window) -> Self {
        let window = Arc::new(window);
        let surface = Surface::new(app.context.as_ref().unwrap(), Arc::clone(&window)).unwrap();

        let size = window.inner_size();
        let mut state = Self {
            surface,
            window,
            cursor_position: Default::default(),
            modifiers: Default::default(),
            cursor_grab: CursorGrabMode::None,
            zoom: 0.,
            rotated: 0.,
            panned: Default::default(),
        };
        state.resize(size);
        state
    }

    /// Resize the window to the new size.
    fn resize(&mut self, size: PhysicalSize<u32>) {
        info!("Resized to {size:?}");
        let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        else {
            return;
        };
        self.surface
            .resize(width, height)
            .expect("failed to resize inner buffer");
        self.window.request_redraw();
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // if self.occluded {
        //     info!("Skipping drawing occluded window={:?}", self.window.id());
        //     return Ok(());
        // }

        const WHITE: u32 = 0x00101010;
        let color = WHITE;
        let mut buffer = self.surface.buffer_mut()?;
        buffer.fill(color);
        self.window.pre_present_notify();
        buffer.present()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum UserEvent {
    WakeUp,
}
