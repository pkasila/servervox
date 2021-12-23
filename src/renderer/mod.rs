use corevox::devices::device::Device;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;

pub struct BaseRenderer {
    pub device_information: DeviceInformation,
}

impl Renderer for BaseRenderer {
    fn handle_vox_pack(&self, pack: VoxPack) {
        let framerate = pack.z * self.device_information.pov_frequency;
        println!("Received {} bytes (framerate needed: {})", pack.raw.capacity(), framerate);
    }
}