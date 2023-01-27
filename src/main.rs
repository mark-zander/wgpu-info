use wgpu::{self, TextureFormat};
use cfg_if;
use env_logger;
use std::env;

fn main() { 
    let args: Vec<String> = env::args().collect();

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }
    if args.len() > 1 && args[1] == "help" {
        println!("Prints the info for each adapter on the system");
        println!("Options:");
        println!("features -    print adapter features");
        println!("limits -      print adapter limits");
        println!("downlevel -   prints info about the adapter");
        println!("\"texture\" -   the texture is any of the TextureFormats");
        println!("              unquoted, except for no Astc formats");
        println!("help -        prints this list");
        println!();
        println!("ex: cargo run -- features limits");
    } else {
        wgpu::Instance::new(wgpu::Backends::all())
            .enumerate_adapters(wgpu::Backends::all())
            .map(|adapter| {
                // Check if this adapter supports our surface
                println!("{:#?}", adapter.get_info());
                if args.len() > 1 && args[1] == "all" {
                    for s in ["features", "limits", "downlevel", "texture"] {
                        printo(&adapter, s);
                    }
                } else {
                    for s in &args[1..] { printo(&adapter, s); }
                }
            })
            .next()
            .unwrap();
    }
}

// printo - print an option
fn printo(adapter: &wgpu::Adapter, s: &str) {
    if s == "features" {
        println!("{:#?}", adapter.features());
    } else if s == "limits" {
        println!("{:#?}", adapter.limits());
    } else if s == "downlevel" {
        println!("{:#?}", adapter.get_downlevel_capabilities());
    } else {
        match texture_from_str(s) {
            Some(x) =>
                println!("{:#?}: {:#?}",
                    x, adapter.get_texture_format_features(x)),
            None => 
                println!(
                    "option {}, is not legal here, use \"help\" for options", s),
        }
    }
}

// fn texture_from_str<E: Error>(s: &str) -> Result<wgpu::TextureFormat, E> {
fn texture_from_str(s: &str) -> Option<wgpu::TextureFormat> {
    let format = match s.to_lowercase().as_str() {
        "r8unorm" => TextureFormat::R8Unorm,
        "r8snorm" => TextureFormat::R8Snorm,
        "r8uint" => TextureFormat::R8Uint,
        "r8sint" => TextureFormat::R8Sint,
        "r16uint" => TextureFormat::R16Uint,
        "r16sint" => TextureFormat::R16Sint,
        "r16unorm" => TextureFormat::R16Unorm,
        "r16snorm" => TextureFormat::R16Snorm,
        "r16float" => TextureFormat::R16Float,
        "rg8unorm" => TextureFormat::Rg8Unorm,
        "rg8snorm" => TextureFormat::Rg8Snorm,
        "rg8uint" => TextureFormat::Rg8Uint,
        "rg8sint" => TextureFormat::Rg8Sint,
        "r32uint" => TextureFormat::R32Uint,
        "r32sint" => TextureFormat::R32Sint,
        "r32float" => TextureFormat::R32Float,
        "rg16uint" => TextureFormat::Rg16Uint,
        "rg16sint" => TextureFormat::Rg16Sint,
        "rg16unorm" => TextureFormat::Rg16Unorm,
        "rg16snorm" => TextureFormat::Rg16Snorm,
        "rg16float" => TextureFormat::Rg16Float,
        "rgba8unorm" => TextureFormat::Rgba8Unorm,
        "rgba8unormsrgb" => TextureFormat::Rgba8UnormSrgb,
        "rgba8snorm" => TextureFormat::Rgba8Snorm,
        "rgba8uint" => TextureFormat::Rgba8Uint,
        "rgba8sint" => TextureFormat::Rgba8Sint,
        "bgra8unorm" => TextureFormat::Bgra8Unorm,
        "bgra8unormsrgb" => TextureFormat::Bgra8UnormSrgb,
        "rgb10a2unorm" => TextureFormat::Rgb10a2Unorm,
        "rg11b10ufloat" => TextureFormat::Rg11b10Float,
        "rg32uint" => TextureFormat::Rg32Uint,
        "rg32sint" => TextureFormat::Rg32Sint,
        "rg32float" => TextureFormat::Rg32Float,
        "rgba16uint" => TextureFormat::Rgba16Uint,
        "rgba16sint" => TextureFormat::Rgba16Sint,
        "rgba16unorm" => TextureFormat::Rgba16Unorm,
        "rgba16snorm" => TextureFormat::Rgba16Snorm,
        "rgba16float" => TextureFormat::Rgba16Float,
        "rgba32uint" => TextureFormat::Rgba32Uint,
        "rgba32sint" => TextureFormat::Rgba32Sint,
        "rgba32float" => TextureFormat::Rgba32Float,
        "depth32float" => TextureFormat::Depth32Float,
        "depth32floatstencil8" => TextureFormat::Depth32FloatStencil8,
        "depth16unorm" => TextureFormat::Depth16Unorm,
        "depth24plus" => TextureFormat::Depth24Plus,
        "depth24plusstencil8" => TextureFormat::Depth24PlusStencil8,
        "rgb9e5ufloat" => TextureFormat::Rgb9e5Ufloat,
        "bc1rgbaunorm" => TextureFormat::Bc1RgbaUnorm,
        "bc1rgbaunormsrgb" => TextureFormat::Bc1RgbaUnormSrgb,
        "bc2rgbaunorm" => TextureFormat::Bc2RgbaUnorm,
        "bc2rgbaunormsrgb" => TextureFormat::Bc2RgbaUnormSrgb,
        "bc3rgbaunorm" => TextureFormat::Bc3RgbaUnorm,
        "bc3rgbaunormsrgb" => TextureFormat::Bc3RgbaUnormSrgb,
        "bc4runorm" => TextureFormat::Bc4RUnorm,
        "bc4rsnorm" => TextureFormat::Bc4RSnorm,
        "bc5rgunorm" => TextureFormat::Bc5RgUnorm,
        "bc5rgsnorm" => TextureFormat::Bc5RgSnorm,
        "bc6hrgbufloat" => TextureFormat::Bc6hRgbUfloat,
        "bc6hrgbfloat" => TextureFormat::Bc6hRgbSfloat,
        "bc7rgbaunorm" => TextureFormat::Bc7RgbaUnorm,
        "bc7rgbaunormsrgb" => TextureFormat::Bc7RgbaUnormSrgb,
        "etc2rgb8unorm" => TextureFormat::Etc2Rgb8Unorm,
        "etc2rgb8unormsrgb" => TextureFormat::Etc2Rgb8UnormSrgb,
        "etc2rgb8a1unorm" => TextureFormat::Etc2Rgb8A1Unorm,
        "etc2rgb8a1unormsrgb" => TextureFormat::Etc2Rgb8A1UnormSrgb,
        "etc2rgba8unorm" => TextureFormat::Etc2Rgba8Unorm,
        "etc2rgba8unormsrgb" => TextureFormat::Etc2Rgba8UnormSrgb,
        "eacr11unorm" => TextureFormat::EacR11Unorm,
        "eacr11snorm" => TextureFormat::EacR11Snorm,
        "eacrg11unorm" => TextureFormat::EacRg11Unorm,
        "eacrg11snorm" => TextureFormat::EacRg11Snorm,
        _ => return None,
        // other => {
        //     if let Some(parts) = other.strip_prefix("astc-") {
        //         let (block, channel) = parts
        //             .split_once('-')
        //             .ok_or_else(|| E::invalid_value(Unexpected::Str(s), &self))?;

        //         let block = match block {
        //             "4x4" => AstcBlock::B4x4,
        //             "5x4" => AstcBlock::B5x4,
        //             "5x5" => AstcBlock::B5x5,
        //             "6x5" => AstcBlock::B6x5,
        //             "6x6" => AstcBlock::B6x6,
        //             "8x5" => AstcBlock::B8x5,
        //             "8x6" => AstcBlock::B8x6,
        //             "8x8" => AstcBlock::B8x8,
        //             "10x5" => AstcBlock::B10x5,
        //             "10x6" => AstcBlock::B10x6,
        //             "10x8" => AstcBlock::B10x8,
        //             "10x10" => AstcBlock::B10x10,
        //             "12x10" => AstcBlock::B12x10,
        //             "12x12" => AstcBlock::B12x12,
        //             _ => return Err(E::invalid_value(Unexpected::Str(s), &self)),
        //         };

        //         let channel = match channel {
        //             "unorm" => AstcChannel::Unorm,
        //             "unorm-srgb" => AstcChannel::UnormSrgb,
        //             "hdr" => AstcChannel::Hdr,
        //             _ => return Err(E::invalid_value(Unexpected::Str(s), &self)),
        //         };

        //         TextureFormat::Astc { block, channel }
        //     } else {
        //         return Err(E::invalid_value(Unexpected::Str(s), &self));
        //     }
        // }
    };

//    Ok(format)
    Some(format)
}
