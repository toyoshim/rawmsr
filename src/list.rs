use usb_ids::FromId;

pub fn run(vid: Option<u16>, pid: Option<u16>) {
    let device_list = match rusb::devices() {
        Ok(device_list) => device_list,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("Found {} devices", device_list.len());
    println!("Filter:");
    if vid.is_some() {
        println!("  Vendor ID {:04X}", vid.unwrap());
    }
    if pid.is_some() {
        println!("  Product ID {:04X}", pid.unwrap());
    }
    println!("Devices:");
    for device in device_list.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(desc) => desc,
            Err(e) => {
                println!("Failed to read the device descriptor: {}", e);
                continue;
            }
        };
        if vid.is_some() && vid.unwrap() != device_desc.vendor_id() {
            continue;
        }
        if pid.is_some() && pid.unwrap() != device_desc.product_id() {
            continue;
        }
        println!(
            "  Bus: {:>2}, Port: {:>2}, Address: {:>3}, Speed: {:?}",
            device.bus_number(),
            device.port_number(),
            device.address(),
            device.speed()
        );
        println!("    Device Descriptor:");
        println!("      USB Version: {}", device_desc.usb_version());
        println!("      Device Version: {}", device_desc.device_version());
        println!(
            "      Vendor ID: {:04X} ({})",
            device_desc.vendor_id(),
            match usb_ids::Vendor::from_id(device_desc.vendor_id()) {
                Some(vendor) => vendor.name(),
                None => "unknown",
            }
        );
        println!(
            "      Product ID: {:04X} ({})",
            device_desc.product_id(),
            match usb_ids::Device::from_vid_pid(device_desc.vendor_id(), device_desc.product_id()) {
                Some(product) => product.name(),
                None => "unknown",
            }
        );
        println!("      Class: {}", device_desc.class_code());
        println!("      Sub Class: {}", device_desc.sub_class_code());
        println!("      Protocol: {}", device_desc.protocol_code());
        println!("      Max Packet Size: {}", device_desc.max_packet_size());
        println!(
            "      Num Configurations: {}",
            device_desc.num_configurations()
        );
        if device_desc.manufacturer_string_index().is_some() {
            println!(
                "      Manufacturer: {}",
                device_desc.manufacturer_string_index().unwrap()
            );
        }
        if device_desc.product_string_index().is_some() {
            println!(
                "      Product: {}",
                device_desc.product_string_index().unwrap()
            );
        }
        if device_desc.serial_number_string_index().is_some() {
            println!(
                "      Serial Number: {}",
                device_desc.serial_number_string_index().unwrap()
            );
        }
        for config_index in 0..device_desc.num_configurations() {
            println!("    Configulation {}:", config_index);
            let config_desc = match device.config_descriptor(config_index) {
                Ok(desc) => desc,
                Err(e) => {
                    println!("    Failed to read the config descriptor: {}", e);
                    continue;
                }
            };
            println!("      Max Power: {}", config_desc.max_power());
            println!("      Self Powered: {}", config_desc.self_powered());
            println!("      Remote Wakeup: {}", config_desc.remote_wakeup());
            println!("      Num of Interfaces: {}", config_desc.num_interfaces());

            for interface in config_desc.interfaces() {
                println!("    Interface {}:", interface.number());
                let desc = interface.descriptors().next().unwrap();
                println!("      Alternative Setting: {}", desc.setting_number());
                println!("      Class: {}", desc.class_code());
                println!("      Sub Class: {}", desc.sub_class_code());
                println!("      Protocol: {}", desc.protocol_code());
                println!("      Num of Endpoints: {}", desc.num_endpoints());
                for ep in desc.endpoint_descriptors() {
                    println!("      Endpoint {} ({:02X}):", ep.number(), ep.address());
                    println!("        Direction: {:?}", ep.direction());
                    println!("        Transfer Type: {:?}", ep.transfer_type());
                    println!("        Sync Type: {:?}", ep.sync_type());
                    println!("        Usage Type: {:?}", ep.usage_type());
                    println!("        Max Packet Size: {}", ep.max_packet_size());
                    println!("        Interval: {}", ep.interval());
                    println!("        Refresh: {}", ep.refresh());
                    println!("        Synch Address: {}", ep.synch_address());
                }
            }
        }
    }
}
