// TODO: [] shapes
// [] collissions
// [] camera
// [] player movement
// [] shaders
use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::Surface,
    Version, VulkanLibrary,
};

use x11;

fn main() {
    println!("Hello, world!");
}

fn vulkano_test() {
    let library = VulkanLibrary::new().unwrap();
    let instance =
        Instance::new(library, InstanceCreateInfo::application_from_cargo_toml()).unwrap();
    let display = x11::xlib::Display;
    let window = x11::xlib::Window;
    unsafe {
        let surface = Surface::from_xlib(instance, display, window, object);
    };
}
