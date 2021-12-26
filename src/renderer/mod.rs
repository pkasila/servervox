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
            .arg("ffmpeg -f mp4 -c:v h264_v4l2m2m -i pipe: -pix_fmt bgra -f rawvideo pipe:")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = t.stdin.take() {
            stdin.write_all(&p.raw).unwrap();
        }

        let mut data: Vec<u8> = vec![];
        t.stdout.unwrap().read_to_end(&mut data).unwrap();

        let mut p = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("ffmpeg -re -f rawvideo -pix_fmt bgra -s {}x{} -r {} -i pipe: -f fbdev /dev/fb0",
                         self.device_information.vox_size[0], self.device_information.vox_size[1],
                         self.device_information.vox_size[2] * self.device_information.pov_frequency))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = p.stdin.take() {
            for _ in 0..10 {
                stdin.write_all(&data).unwrap();
            }
        }
    }
}