use clap::Parser;

mod check;
mod list;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[arg(short, long, help = "Check underlying libusb support")]
    check: bool,

    #[arg(short, long, help = "List detected USB devices")]
    list: bool,

    #[arg(short, long, help = "Target devices' Vendor ID")]
    vid: Option<String>,

    #[arg(short, long, help = "Target devices' Product ID")]
    pid: Option<String>,
}

fn main() {
    let options = Options::parse();

    let vid: Option<u16> = match options.vid {
        None => None,
        Some(vid) => match u16::from_str_radix(&vid, 16) {
            Err(_e) => None,
            Ok(vid) => Some(vid),
        },
    };
    let pid: Option<u16> = match options.pid {
        None => None,
        Some(pid) => match u16::from_str_radix(&pid, 16) {
            Err(_e) => None,
            Ok(pid) => Some(pid),
        },
    };

    if vid.is_some() && pid.is_some() {
        let handle = rusb::open_device_with_vid_pid(vid.unwrap(), pid.unwrap());
        match handle {
            None => {
                println!("fail")
            }
            Some(_handle) => {
                println!("ok")
            }
        }
        return;
    }

    if options.check {
        check::run();
    }
    if options.list {
        list::run(vid, pid);
    }
}
