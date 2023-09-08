#[cfg(target_os = "windows")]
#[macro_use]
extern crate windows_service;

mod api;
mod command;
mod server;
mod state;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn main() -> Result<(), windows_service::Error> {
    windows::main()
}

#[cfg(not(target_os = "windows"))]
pub fn main() {
    crate::server::serve(None);
}
