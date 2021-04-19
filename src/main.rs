use odbc_api::Environment;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    // Create a temporary parent window
    let window = WindowBuilder::new().build(&EventLoop::new()).unwrap();
    let environment = unsafe { Environment::new().unwrap() };
    let (_connection, connection_string) = environment
        .connect_with_complete_prompt("", Some(&window), 1024)
        .unwrap();
    println!("{}", connection_string);
}
