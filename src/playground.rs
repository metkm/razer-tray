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

    let mut offset = 0;
    let mut data: Vec<u8> = Vec::with_capacity(10 * 11);

    loop {
        let written_count = write(&hid_device, 0x08, Some(offset));
        let response_buff = &read(&hid_device, written_count)[1..];

        data.extend_from_slice(&response_buff[5..]);
        println!("{:?}", &response_buff[5..]);

        let (new_offset, overflowed) = offset.overflowing_add(10);

        if overflowed {
            break;
        }

        offset = new_offset
    }
}
