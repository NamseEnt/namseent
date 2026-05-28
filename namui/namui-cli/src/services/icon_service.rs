use anyhow::{Result, anyhow, bail};
use image::{ImageBuffer, Rgba, imageops::FilterType};
use std::path::{Path, PathBuf};

const ICO_SIZES: [u32; 6] = [16, 32, 48, 64, 128, 256];
const SHORTCUT_ICON_SIZE: u32 = 512;
const APP_ICON_SIZE: u32 = 184;
const APP_ICON_JPEG_QUALITY: u8 = 90;

pub fn read_icon_path(manifest_path: &Path) -> Result<Option<PathBuf>> {
    let manifest_abs = std::fs::canonicalize(manifest_path)
        .map_err(|e| anyhow!("canonicalize {:?}: {e}", manifest_path))?;
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&manifest_abs)
        .no_deps()
        .exec()?;
    let pkg = metadata
        .packages
        .iter()
        .find(|p| std::path::Path::new(&p.manifest_path) == manifest_abs)
        .ok_or_else(|| anyhow!("package not found for manifest {:?}", manifest_abs))?;
    let icon = pkg
        .metadata
        .get("namui")
        .and_then(|n| n.get("icon"))
        .and_then(|v| v.as_str());
    let Some(icon) = icon else {
        return Ok(None);
    };
    let project_dir = manifest_abs.parent().unwrap();
    Ok(Some(project_dir.join(icon)))
}

pub fn validate_source(src: &Path) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    if !src.exists() {
        bail!("icon source not found: {:?}", src);
    }
    let img = image::open(src)
        .map_err(|e| anyhow!("failed to decode icon {:?}: {e}", src))?
        .to_rgba8();
    let (w, h) = (img.width(), img.height());
    if w != h {
        bail!("icon must be square, got {w}x{h} at {:?}", src);
    }
    if w < 256 {
        bail!("icon must be at least 256x256, got {w}x{h} at {:?}", src);
    }
    if w < 1024 {
        eprintln!(
            "warning: icon {:?} is {w}x{w}; recommend >=1024 for crisp macOS/Steam assets",
            src
        );
    }
    Ok(img)
}

pub fn generate_ico(src_image: &ImageBuffer<Rgba<u8>, Vec<u8>>, dst: &Path) -> Result<()> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for size in ICO_SIZES {
        let resized = image::imageops::resize(src_image, size, size, FilterType::Lanczos3);
        let ico_image = ico::IconImage::from_rgba_data(size, size, resized.into_raw());
        let entry = ico::IconDirEntry::encode(&ico_image)
            .map_err(|e| anyhow!("ico encode {size}: {e}"))?;
        icon_dir.add_entry(entry);
    }
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(dst)
        .map_err(|e| anyhow!("create ico {:?}: {e}", dst))?;
    icon_dir
        .write(file)
        .map_err(|e| anyhow!("write ico {:?}: {e}", dst))?;
    Ok(())
}

pub fn generate_steam_assets(
    src_image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    dst_dir: &Path,
) -> Result<()> {
    std::fs::create_dir_all(dst_dir)?;

    let shortcut =
        image::imageops::resize(src_image, SHORTCUT_ICON_SIZE, SHORTCUT_ICON_SIZE, FilterType::Lanczos3);
    let shortcut_path = dst_dir.join("shortcut-icon-512.png");
    shortcut
        .save_with_format(&shortcut_path, image::ImageFormat::Png)
        .map_err(|e| anyhow!("save shortcut icon {:?}: {e}", shortcut_path))?;

    let app_rgba =
        image::imageops::resize(src_image, APP_ICON_SIZE, APP_ICON_SIZE, FilterType::Lanczos3);
    let mut rgb = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(APP_ICON_SIZE, APP_ICON_SIZE);
    for (x, y, src_pixel) in app_rgba.enumerate_pixels() {
        let [r, g, b, a] = src_pixel.0;
        let alpha = a as u32;
        let blend = |c: u8| ((c as u32 * alpha) / 255) as u8;
        rgb.put_pixel(x, y, image::Rgb([blend(r), blend(g), blend(b)]));
    }
    let app_path = dst_dir.join("app-icon-184.jpg");
    let mut app_file = std::fs::File::create(&app_path)
        .map_err(|e| anyhow!("create app icon {:?}: {e}", app_path))?;
    let mut encoder =
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut app_file, APP_ICON_JPEG_QUALITY);
    encoder
        .encode_image(&rgb)
        .map_err(|e| anyhow!("encode app icon {:?}: {e}", app_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tower_defense_icon_src() -> PathBuf {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .join("../../tower-defense/asset/image/tower/high/idle1.png")
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn generates_ico_and_steam_assets() {
        let src = tower_defense_icon_src();
        let img = validate_source(&src).unwrap();

        let tmp = std::env::temp_dir().join("namui_cli_icon_test");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let ico_path = tmp.join("icon.ico");
        generate_ico(&img, &ico_path).unwrap();
        let ico_bytes = std::fs::read(&ico_path).unwrap();
        assert!(ico_bytes.len() > 100, "ico too small");
        assert_eq!(&ico_bytes[0..4], &[0x00, 0x00, 0x01, 0x00], "ico magic");

        generate_steam_assets(&img, &tmp).unwrap();
        let shortcut = image::open(tmp.join("shortcut-icon-512.png")).unwrap();
        assert_eq!(shortcut.width(), 512);
        assert_eq!(shortcut.height(), 512);
        let app_icon = image::open(tmp.join("app-icon-184.jpg")).unwrap();
        assert_eq!(app_icon.width(), 184);
        assert_eq!(app_icon.height(), 184);
    }
}
