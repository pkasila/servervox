use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use corevox::devices::device::Device;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;

pub struct BaseRenderer {
    pub device_information: DeviceInformation,
    ffmpeg_process: Option<Child>,
}

impl BaseRenderer {
    pub fn new(device_information: DeviceInformation) -> Self {
        let mut r = BaseRenderer {
            device_information,
            ffmpeg_process: None,
        };

        r.start_daemon();

        return r;
    }

    fn start_daemon(&mut self) {
        self.ffmpeg_process = match Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("ffmpeg -f rawvideo -pix_fmt rgb565le -s {}x{} -r 168 -i pipe: -pix_fmt bgra -f fbdev /dev/fb0",
                         self.device_information.frame_size[0], self.device_information.frame_size[1]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
            Ok(process) => Some(process),
        };
    }
}

impl Renderer for BaseRenderer {
    fn handle_vox_pack(&self, pack: VoxPack) {
        let mut p = pack;
        println!("Received {} bytes", p.raw.len());

        let mut t = Command::new("/bin/sh")
            .arg("-c")
            .arg("ffmpeg -f mp4 -c:v h264_v4l2m2m -i pipe: -pix_fmt rgb565le -f rawvideo pipe:")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = t.stdin.take() {
            stdin.write_all(&p.raw).unwrap();
        }

        let mut data: Vec<u8> = vec![];
        t.stdout.unwrap().read_to_end(&mut data).unwrap();

        let framerate = p.z * self.device_information.pov_frequency;

        let size = self.device_information.frame_size[0] * self.device_information.frame_size[1] * 2;

        for _ in 0..self.device_information.pov_frequency {
            for chunk in data.chunks(size as usize) {
                self.ffmpeg_process.as_ref().unwrap().stdin.as_ref()
                    .unwrap().write(chunk);
                thread::sleep(Duration::new(0, (1 * 1000000000 / framerate) as u32));
            }
            for chunk in data.chunks(size as usize).rev() {
                self.ffmpeg_process.as_ref().unwrap().stdin.as_ref()
                    .unwrap().write(chunk);
                thread::sleep(Duration::new(0, (1 * 1000000000 / framerate) as u32));
            }
        }
    }
}