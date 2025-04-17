use hidapi::{HidApi, HidDevice};

fn get_usb_crc(value: &[u8]) -> i32 {
    let mut value_slice = value.iter();
    value_slice.next_back();

    let mut crc = value_slice.fold(0_i32, |acc, i| acc + (*i as i32));

    crc = if crc > 255 { crc - 256 } else { crc };
    crc = 0x55 - crc;

    return crc;
}

fn write(hid_device: &HidDevice, command: u8, offset: Option<u8>) -> usize {
    let mut buffer: Vec<u8> = vec![
        0x08, command, 0x00, 0x00, offset.unwrap_or(0x00), 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xef,
    ];

    let crc = get_usb_crc(&buffer[1..]);

    buffer
        .get_mut(16)
        .and_then(|val| {
            *val = (crc - 0x08) as u8;
            Some(val)
        });

    let written_count = hid_device.write(&buffer).unwrap();
    written_count
}

fn read(hid_device: &HidDevice, len: usize) -> Vec<u8> {
    let mut response_buff = Vec::with_capacity(len);
    unsafe { response_buff.set_len(len - 1) };

    hid_device.read(&mut response_buff).unwrap();
    response_buff
}

pub fn get_battery() -> Option<u8> {
    let api = HidApi::new().unwrap();

    let Some(device) = api
        .device_list()
        .skip(2)
        .find(|device| device.vendor_id() == 13652)
    else {
        return None;
    };

    let hid_device = device.open_device(&api).unwrap();
    
    let written_count = write(&hid_device, 0x04, None);
    let response_buff = read(&hid_device, written_count);

    let battery = response_buff.get(6)
        .map(|x| *x);

    battery
    
    /////////
    // Old razer code
    /////////

    // let Some(mouse) = rusb::devices()?.iter().find(|device| {
    //     let device_desc = device.device_descriptor().unwrap();
    //     let vendor_hex = format!("{:x}", device_desc.vendor_id());

    //     vendor_hex == "1532"
    // }) else {
    //     return Err(rusb::Error::NoDevice);
    // };

    // let mut request = RazerReport::new(0x1f, 0x07, 0x80);
    // let mut request_bytes = unsafe { struct_to_bytes(&mut request) };

    // let mut crc = 0;

    // for i in &request_bytes[2..] {
    //     crc ^= i;
    // }

    // request.crc = crc;
    // request_bytes = unsafe { struct_to_bytes(&mut request) };

    // let device_handle = mouse.open().unwrap();
    // let write_request_type = rusb::request_type(
    //     rusb::Direction::Out,
    //     rusb::RequestType::Class,
    //     rusb::Recipient::Interface,
    // );

    // device_handle.write_control(
    //     write_request_type,
    //     0x09,
    //     0x300,
    //     0x00,
    //     &request_bytes,
    //     Duration::from_secs(1),
    // )?;

    // std::thread::sleep(Duration::from_millis(100));

    // let read_request_type = rusb::request_type(
    //     rusb::Direction::In,
    //     rusb::RequestType::Class,
    //     rusb::Recipient::Interface,
    // );

    // device_handle.read_control(
    //     read_request_type,
    //     0x01,
    //     0x300,
    //     0x00,
    //     request_bytes,
    //     Duration::from_secs(1),
    // )?;

    // let arguments_offset = offset_of!(RazerReport, arguments);
    // let battery_level = *request_bytes.get(arguments_offset + 1).unwrap() as f32;
    // let battery_level_normalized = (battery_level / 255.0) * 100.0;

    // Ok(battery_level_normalized)
}
