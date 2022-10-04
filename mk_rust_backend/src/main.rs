pub mod electron_js_socket;
pub mod event_management;
use std::thread;

fn launch_threads(){
    thread::spawn(||electron_js_socket::server_socket_thread);
}

fn main() {
    launch_threads();
}