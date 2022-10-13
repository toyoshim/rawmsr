pub fn run() {
    println!("libusb support check:");
    let version = rusb::version();
    println!(
        "  version: {}.{}.{}.{}{}",
        version.major(),
        version.minor(),
        version.micro(),
        version.nano(),
        version.rc().unwrap_or("")
    );
    println!("  capability: {}", rusb::has_capability());
    println!("  hid_access: {}", rusb::has_hid_access());
    println!("  hotplug: {}", rusb::has_hotplug());
    println!(
        "  detach kernel driver: {}",
        rusb::supports_detach_kernel_driver()
    );
}
