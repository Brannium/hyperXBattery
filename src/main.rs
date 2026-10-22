use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    os::fd::AsRawFd,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    process,
};

const DEVICE_ID: &str = "000003F0:0000098D";
const CMD_PREFIX: [u8; 3] = [0x21, 0xBB, 0x0B];
const POLL_TIMEOUT_MS: i32 = 1000;
const BUFFER_SIZE: usize = 64;
const CMD_SIZE: usize = 31;
const RESPONSE_PATTERN_SIZE: usize = 4;
const RESPONSE_VALUE_OFFSET: usize = 3;

fn main() {
    let Ok(entries) = fs::read_dir("/sys/class/hidraw") else {
        eprintln!("Failed to read /sys/class/hidraw: check permissions or kernel module");
        process::exit(1);
    };

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let name = entry.file_name();

        let uevent = fs::read_to_string(entry.path().join("device/uevent"))
            .unwrap_or_default();

        if !uevent.contains(DEVICE_ID) {
            continue;
        }

        let device = Path::new("/dev").join(name);
        let Ok(mut file) = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(device)
        else {
            continue;
        };

        let mut cmd = [0u8; CMD_SIZE];
        cmd[..CMD_PREFIX.len()].copy_from_slice(&CMD_PREFIX);

        if file.write_all(&cmd).is_err() {
            continue;
        }

        let mut pollfd = libc::pollfd {
            fd: file.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };

        if unsafe { libc::poll(&mut pollfd, 1, POLL_TIMEOUT_MS) } <= 0 {
            continue;
        }

        let mut buf = [0u8; BUFFER_SIZE];
        let Ok(n) = file.read(&mut buf) else {
            continue;
        };

        if let Some(i) = buf[..n]
            .windows(RESPONSE_PATTERN_SIZE)
            .position(|w| {
                w[0] == CMD_PREFIX[0]
                    && w[1] == CMD_PREFIX[1]
                    && w[2] == CMD_PREFIX[2]
            })
        {
            println!("{}%", buf[i + RESPONSE_VALUE_OFFSET]);
            return;
        }
    }

    eprintln!("No response: check dongle, headset power, and permissions");
    process::exit(1);
}
