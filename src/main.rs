use wgpu::winit::{
  ElementState,
  Event,
  EventsLoop,
  KeyboardInput,
  VirtualKeyCode,
  WindowEvent
};

struct Vertex {
  pos: [f32; 2],
}

fn main() {
  let mut events_loop = EventsLoop::new();

  let (_window, instance, hidpi_factor, size, surface) = {
    use wgpu::winit::Window;

    let instance = wgpu::Instance::new();
    let window = Window::new(&events_loop).unwrap();
    window.set_title("lacknafta");
    let hidpi_factor = window.get_hidpi_factor();
    let size = window.get_inner_size().unwrap().to_physical(hidpi_factor);

    let surface = instance.create_surface(&window);

    (window, instance, hidpi_factor, size, surface)
  };

  let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
    power_preference: wgpu::PowerPreference::HighPerformance
  });

  let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
    extensions: wgpu::Extensions { anisotropic_filtering: false }
  });

  let mut sc_desc = wgpu::SwapChainDescriptor {
    usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
    format: wgpu::TextureFormat::Bgra8UnormSrgb,
    width: size.width.round() as u32,
    height: size.height.round() as u32,
  };
}
