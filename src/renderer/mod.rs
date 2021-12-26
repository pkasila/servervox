use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use corevox::devices::device::Device;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;

pub struct BaseRenderer {
    pub device_information: DeviceInformation,
}

impl BaseRenderer {
    pub fn new(device_information: DeviceInformation) -> Self {
        let mut r = BaseRenderer {
            device_information,
        };

        return r;
    }
}

impl Renderer for BaseRenderer {
    fn handle_vox_pack(&self, pack: VoxPack) {
        let mut p = pack;
        println!("Received {} bytes", p.raw.len());

        let mut t = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("ffmpeg -loop {} -r {} -f mp4 -c:v h264_v4l2m2m -i pipe: -pix_fmt bgra -f fbdev /dev/fb0",
                         self.device_information.pov_frequency * 10,
                         self.device_information.pov_frequency * self.device_information.vox_size[2]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = t.stdin.take() {
            stdin.write_all(&p.raw).unwrap();
        }
    }
}