use std::{fmt::format, time::Duration};

use hidapi::{DeviceInfo, HidApi};

fn get_usb_crc(value: &[u8]) -> i32 {
    let mut value_slice = value.iter();
    value_slice.next_back();

    let mut crc = value_slice.fold(0_i32, |acc, i| acc + (*i as i32));

    crc = if crc > 255 { crc - 256 } else { crc };

    crc = 0x55 - crc;

    return crc;
}

fn main() {
    let api = HidApi::new().unwrap();

    let Some(device) = api
        .device_list()
        .filter(|device| device.vendor_id() == 13652)
        .skip(1)
        .find(|device| device.vendor_id() == 13652)
    else {
        return;
    };

    let hid_device = device.open_device(&api).unwrap();

    let mut buffer: Vec<u8> = vec![
        0x08, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
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

    let mut response_buff = Vec::with_capacity(written_count);
    unsafe { response_buff.set_len(written_count - 1) };

    hid_device.read(&mut response_buff).unwrap();

    let v1 = response_buff.get(6)
        .unwrap_or(&0);

    let v2 = response_buff.get(7)
        .and_then(|item| Some(format!("{:#x}", item)))
        .unwrap_or_default();

    println!("{:?} - {:?} - {:?}", response_buff, v1, v2);

    // let mut add: u8 = 0;
    // loop {
    //     buffer
    //         .get_mut(3)
    //         .and_then(|val| {
    //             *val = add;
    //             Some(val)
    //         });

    //     let crc = get_usb_crc(&buffer);

    //     buffer
    //         .get_mut(15)
    //         .and_then(|val| {
    //             *val = (crc - 0x08) as u8;
    //             Some(val)
    //         });

    //     let written_count = hid_device.write(&buffer)
    //         .unwrap();

    //     std::thread::sleep(Duration::from_millis(40));

    //     let mut response_buff = Vec::with_capacity(written_count);
    //     unsafe { response_buff.set_len(written_count - 1) };

    //     hid_device.read(&mut response_buff).unwrap();

    //     println!("{:?}", response_buff);

    //     break;
    //     add += 10;
    //     if add > 100 {
    //         break;
    //     }
    // }
}
