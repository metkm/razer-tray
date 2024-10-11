mod battery;

use battery::get_battery;
use image::GenericImageView;
use tray_icon::TrayIconBuilder;
use tao::event_loop::EventLoop;

fn main() {
    let icon= include_bytes!("../trayicon.jpg");

    let img = image::load_from_memory(icon).unwrap();
    let img_raw = img.to_rgba8().into_raw();
    let (img_w, img_h) = img.dimensions();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Razer Tray")
        .with_icon(tray_icon::Icon::from_rgba(img_raw, img_w, img_h).unwrap())
        .build()
        .unwrap();

    let event_loop = EventLoop::new();

    event_loop.run(move |_, _, _| {
        let Ok(battery) = get_battery() else {
            return;
        };

        let text = format!("Batter Level {}%", battery as u8);
        tray_icon.set_tooltip(Some(text)).ok();

        std::thread::sleep(std::time::Duration::from_secs(30));
    });
}
