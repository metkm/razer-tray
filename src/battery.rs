use std::{mem::offset_of, time::Duration};

#[repr(packed)]
#[derive(Debug)]
#[allow(dead_code)]
struct RazerReport {
    status: u8,
    transaction_id: u8,
    remaining_packets: u16,
    protocol_type: u8,
    data_size: u8,
    command_class: u8,
    command_id: u8,
    arguments: [i8; 80],
    crc: u8,
    reserved: u8,
}

impl RazerReport {
    fn new(transaction_id: u8, command_class: u8, command_id: u8) -> Self {
        Self {
            status: 0x00,
            transaction_id,
            remaining_packets: 0x00,
            protocol_type: 0x00,
            command_class,
            command_id,
            data_size: 0x80,
            arguments: [0; 80],
            crc: 0x00,
            reserved: 0x00
        }
    }
}

unsafe fn struct_to_bytes<T>(p: &mut T) -> &mut [u8] {
    ::core::slice::from_raw_parts_mut(
        (p as *mut T) as *mut u8,
        ::core::mem::size_of::<T>()
    )
}

pub fn get_battery() -> rusb::Result<f32> {
    let Some(mouse) = rusb::devices()?.iter().find(|device| {
        let device_desc = device.device_descriptor().unwrap();
        let vendor_hex = format!("{:x}", device_desc.vendor_id());

        vendor_hex == "1532"
    }) else {
        return Err(rusb::Error::NoDevice);
    };

    let mut request = RazerReport::new(0x1f, 0x07, 0x80);
    let mut request_bytes = unsafe { struct_to_bytes(&mut request) };

    let mut crc = 0;

    for i in &request_bytes[2..] {
        crc ^= i;
    }

    request.crc = crc;
    request_bytes = unsafe { struct_to_bytes(&mut request) };

    let device_handle = mouse.open().unwrap();
    let write_request_type = rusb::request_type(
        rusb::Direction::Out,
        rusb::RequestType::Class,
        rusb::Recipient::Interface,
    );

    device_handle.write_control(
        write_request_type,
        0x09,
        0x300,
        0x00,
        &request_bytes,
        Duration::from_secs(1),
    )?;

    std::thread::sleep(Duration::from_millis(100));

    let read_request_type = rusb::request_type(
        rusb::Direction::In,
        rusb::RequestType::Class,
        rusb::Recipient::Interface,
    );

    device_handle.read_control(
        read_request_type,
        0x01,
        0x300,
        0x00,
        request_bytes,
        Duration::from_secs(1),
    )?;

    let arguments_offset = offset_of!(RazerReport, arguments);
    let battery_level = *request_bytes.get(arguments_offset + 1).unwrap() as f32;
    let battery_level_normalized = (battery_level / 255.0) * 100.0;

    Ok(battery_level_normalized)
}

