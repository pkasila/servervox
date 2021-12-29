use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;
use framebuffer::{Framebuffer, KdMode};

pub struct BaseRenderer {
    pub device_information: DeviceInformation,
}

impl BaseRenderer {
    pub fn new(device_information: DeviceInformation) -> Self {
        return BaseRenderer {
            device_information,
        };
    }
}

impl Renderer for BaseRenderer {
    fn handle_vox_pack(&self, pack: VoxPack) {
        println!("Received {} bytes", pack.raw.len());

        let mut t = Command::new("/bin/sh")
            .arg("-c")
            .arg("ffmpeg -f mp4 -c:v h264_v4l2m2m -i pipe: -pix_fmt bgra -f rawvideo pipe:")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = t.stdin.take() {
            stdin.write_all(&pack.raw).unwrap();
        }

        let mut data: Vec<u8> = vec![];
        t.stdout.unwrap().read_to_end(&mut data).unwrap();

        Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
        let mut fb = Framebuffer::new(Path::new("/dev/fb0")).unwrap();

        for _ in 0..10 * self.device_information.pov_frequency {
            for chunk in data.chunks((4 * self.device_information.vox_size[0] * self.device_information.vox_size[1]) as usize) {
                fb.write_frame(chunk);
                sleep(Duration::new(0, (1000000000 / (self.device_information.pov_frequency * self.device_information.vox_size[2])) as u32))
            }
        }

        Framebuffer::set_kd_mode(KdMode::Text).unwrap();
    }
}