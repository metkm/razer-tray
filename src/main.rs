#![windows_subsystem = "windows"]

mod battery;

use battery::get_battery;
use image::GenericImageView;
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::{menu::MenuEvent, TrayIconBuilder};

fn main() {
    let icon = include_bytes!("../trayicon.jpg");

    let img = image::load_from_memory(icon).unwrap();
    let img_raw = img.to_rgba8().into_raw();
    let (img_w, img_h) = img.dimensions();

    let quit_item = tray_icon::menu::MenuItem::with_id("quit", "quit", true, None);
    let menu = tray_icon::menu::Menu::with_items(&[&quit_item]).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Razer Tray")
        .with_menu(Box::new(menu))
        .with_icon(tray_icon::Icon::from_rgba(img_raw, img_w, img_h).unwrap())
        .build()
        .unwrap();

    let event_loop = EventLoop::new();

    let mut battery_level_text = String::new();

    std::thread::scope(|s| {
        s.spawn(|| {
            let battery = get_battery().unwrap() as u8;
            battery_level_text = format!("Batter Level {}%", battery as u8);
        });
    });

    event_loop.run(move |_, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(MenuEvent { id }) = MenuEvent::receiver().try_recv() {
            let quit_id = quit_item.id();

            if id == quit_id {
                std::process::exit(0);
            }
        }

        tray_icon.set_tooltip(Some(&battery_level_text)).ok();
    });
}
