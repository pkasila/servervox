use corevox::devices::device::Device;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;

pub struct BaseRenderer {
    pub device_information: Option<DeviceInformation>,
}

impl Renderer for BaseRenderer {
    fn set_device_information(&mut self, info: DeviceInformation) {
        self.device_information = Some(info);
    }

    fn handle_vox_pack(&self, pack: VoxPack) {
        let framerate = pack.z * self.device_information.unwrap().pov_frequency;
        println!("Received {} bytes (framerate needed: {})", pack.raw.capacity(), framerate);
    }
}