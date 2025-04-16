use hidapi::HidApi;

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
    println!("{:?}", hid_device);

    let mut buff = vec![
        0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];

    let mut crc = buff
        .iter()
        .fold(0, |acc, i| acc + i);

    crc = crc & 0xff;
    crc = 0x55 - crc;

    buff.push(crc);

    let written_count = hid_device.write(&buff).unwrap();

    let mut response_buff: Vec<u8> = Vec::with_capacity(16);
    unsafe {
        response_buff.set_len(written_count);
    }

    hid_device.read(&mut response_buff).unwrap();

    let battery = response_buff.get(6).unwrap().to_owned();

    Some(battery)

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
