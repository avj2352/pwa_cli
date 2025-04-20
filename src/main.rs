use anyhow::Result;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Icon {
    src: String,
    sizes: String,
    r#type: String,
    purpose: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    icons: Vec<Icon>,
    // ... other manifest fields
}

fn main() -> Result<()> {
    let source_path = "Favicon.png";
    let output_dir = "public/icons";
    fs::create_dir_all(output_dir)?;

    let sizes = [48, 72, 96, 128, 192, 256, 384, 512];

    let img = image::open(source_path)?;

    let mut icons = Vec::new();

    for size in &sizes {
        let resized = img.resize_exact(*size, *size, image::imageops::Lanczos3);
        let filename = format!("{}/icon-{}x{}.png", output_dir, size, size);
        resized.save_with_format(&filename, ImageFormat::Png)?;

        icons.push(Icon {
            src: filename.clone(),
            sizes: format!("{}x{}", size, size),
            r#type: "image/png".to_string(),
            purpose: Some("any".to_string()),
        });
    }

    // Update manifest.json
    let manifest_path = "public/manifest.json";
    let mut manifest: Manifest = if Path::new(manifest_path).exists() {
        let data = fs::read_to_string(manifest_path)?;
        serde_json::from_str(&data)?
    } else {
        Manifest { icons: Vec::new() }
    };

    manifest.icons = icons;

    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    let mut file = File::create(manifest_path)?;
    file.write_all(manifest_json.as_bytes())?;

    println!("PWA assets generated successfully.");
    Ok(())
}
