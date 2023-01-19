use wgpu;
use cfg_if;
use env_logger;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    // if wgpu::TextureFormat::into("Rgba8UnormSrgb") == wgpu::TextureFormat::Rgba8UnormSrgb {
    //     println!("works");
    // }

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    instance
        .enumerate_adapters(wgpu::Backends::all())
        .map(|adapter| {
            // Check if this adapter supports our surface
            println!("{:#?}", adapter.get_info());

            for i in 1..args.len() {
                let s = &args[i];
                if s == "features" {
                    println!("{:#?}", adapter.features());
                } else if s == "limits" {
                    println!("{:#?}", adapter.limits());
                } else if s == "downlevel" {
                    println!("{:#?}", adapter.get_downlevel_capabilities());
                } else if s == "texture" {
                    println!("{:#?}", adapter.get_texture_format_features(
                        wgpu::TextureFormat::Rgba8UnormSrgb));
                } else {
                    println!("Prints the info for each adapter on the system");
                    println!("Options:");
                    println!("features -  print adapter features");
                    println!("limits -    print adapter limits");
                    println!("downlevel - prints info about the adapter");
                    println!("texture -   prints texture format for Rgba8UnormSrgb");
                    println!("help -      prints this list");
                    println!("");
                    println!("ex: cargo run -- features limts");
                }
            }
        })
        .next()
        .unwrap();
}
