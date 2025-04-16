use hidapi::HidApi;

fn get_usb_crc(value: &[u8]) -> i32 {
    let mut value_slice = value.iter();
    value_slice.next_back();

    let mut crc = value_slice
        .fold(0_i32, |acc, i| acc + (*i as i32));

    crc = if crc > 255 {
        crc - 256
    } else {
        crc
    };

    crc = 0x55 - crc;

    return crc
}

fn main() {
    let api = HidApi::new().unwrap();

    let Some(device) = api
        .device_list()
        .skip(2)
        .find(|device| device.vendor_id() == 13652)
    else {
        return;
    };

    let hid_device = device
        .open_device(&api)
        .unwrap();

    let mut buffer: Vec<u8> = vec![0x08, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef];

    let mut add: u8 = 0;
    loop {
        buffer
            .get_mut(3)
            .and_then(|val| {
                *val = add;
                Some(val)
            });

        let crc = get_usb_crc(&buffer);
        println!("crc - {:?}", crc);

        buffer
            .get_mut(15)
            .and_then(|val| {
                *val = (crc - 0x08) as u8;
                Some(val)
            });

        let write_count = hid_device.write(&buffer)
            .unwrap();

        println!("{:?}", buffer);

        add += 10;

        if add > 100 {
            break;
        }
    }
}
