mod commands {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::path::{Path, PathBuf};
    #[cfg(target_os = "macos")]
    use std::process::Command;
    use tempfile::TempDir;
    use zip::write::FileOptions;
    use chrono::Local;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HeicFileInfo {
        pub path: String,
        pub name: String,
        pub width: u32,
        pub height: u32,
        pub size_bytes: u64,
        pub thumbnail: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConversionResult {
        pub success: bool,
        pub output_path: String,
        pub message: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SizeEstimate {
        pub original_total: u64,
        pub estimated_total: u64,
        pub savings_percent: f32,
    }

    fn is_heic_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "heic" | "heif")
        } else {
            false
        }
    }

    fn extract_heic_from_zip(zip_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let file = File::open(zip_path)?;
        let mut archive = zip::ZipArchive::new(BufReader::new(file))?;
        let temp_dir = TempDir::new()?;
        let mut extracted = Vec::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            if entry.is_dir() { continue; }
            let path = Path::new(entry.name());
            if is_heic_file(path) {
                let extract_path = temp_dir.path().join(path.file_name().unwrap());
                let mut outfile = File::create(&extract_path)?;
                let mut buffer = vec![];
                entry.read_to_end(&mut buffer)?;
                outfile.write_all(&buffer)?;
                extracted.push(extract_path);
            }
        }
        let _ = Box::leak(Box::new(temp_dir));
        Ok(extracted)
    }

    #[cfg(target_os = "macos")]
    fn get_image_dimensions(path: &Path) -> anyhow::Result<(u32, u32)> {
        let output = Command::new("sips")
            .args(["-g", "pixelWidth", "-g", "pixelHeight", &path.to_string_lossy()])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut width = 0u32;
        let mut height = 0u32;
        for line in stdout.lines() {
            if line.contains("pixelWidth:") {
                if let Some(val) = line.split(':').nth(1) {
                    width = val.trim().parse().unwrap_or(0);
                }
            }
            if line.contains("pixelHeight:") {
                if let Some(val) = line.split(':').nth(1) {
                    height = val.trim().parse().unwrap_or(0);
                }
            }
        }
        Ok((width, height))
    }

    #[cfg(not(target_os = "macos"))]
    fn get_image_dimensions(_path: &Path) -> anyhow::Result<(u32, u32)> {
        Ok((0, 0))
    }

    #[cfg(target_os = "macos")]
    fn generate_thumbnail_base64(path: &Path, max_size: u32) -> anyhow::Result<String> {
        let temp_dir = TempDir::new()?;
        let thumb_path = temp_dir.path().join("thumb.jpg");
        let status = Command::new("sips")
            .args(["-Z", &max_size.to_string(), "-s", "format", "jpeg",
                   &path.to_string_lossy(), "--out", &thumb_path.to_string_lossy()])
            .status()?;
        if !status.success() {
            anyhow::bail!("sips failed");
        }
        let thumb_data = std::fs::read(&thumb_path)?;
        Ok(format!("data:image/jpeg;base64, {}", base64_encode(&thumb_data)))
    }

    #[cfg(not(target_os = "macos"))]
    fn generate_thumbnail_base64(_path: &Path, _max_size: u32) -> anyhow::Result<String> {
        anyhow::bail!("Thumbnail generation not supported")
    }

    #[cfg(target_os = "macos")]
    fn convert_heic_to_jpeg(heic_path: &Path, quality: u8) -> anyhow::Result<Vec<u8>> {
        let temp_dir = TempDir::new()?;
        let output_path = temp_dir.path().join("output.jpg");
        let status = Command::new("sips")
            .args(["-s", "format", "jpeg", "-s", "formatOptions", &quality.to_string(),
                   &heic_path.to_string_lossy(), "--out", &output_path.to_string_lossy()])
            .status()?;
        if !status.success() {
            anyhow::bail!("sips conversion failed");
        }
        let jpeg_data = std::fs::read(&output_path)?;
        Ok(jpeg_data)
    }

    #[cfg(not(target_os = "macos"))]
    fn convert_heic_to_jpeg(_heic_path: &Path, _quality: u8) -> anyhow::Result<Vec<u8>> {
        anyhow::bail!("HEIC conversion requires platform-specific tools")
    }

    fn get_heic_info_internal(path: &Path) -> Result<HeicFileInfo, String> {
        let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
        let size_bytes = metadata.len();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let (width, height) = get_image_dimensions(path).unwrap_or((0, 0));
        let thumbnail = generate_thumbnail_base64(path, 120).ok();
        Ok(HeicFileInfo {
            path: path.to_string_lossy().to_string(),
            name, width, height, size_bytes, thumbnail,
        })
    }

    /// Estimate output size based on quality and original file
    /// This uses an approximation formula based on typical HEIC to JPEG conversion ratios
    fn estimate_jpeg_size(original_size: u64, quality: u8, width: u32, height: u32) -> u64 {
        if width == 0 || height == 0 {
            // Fallback: use ratio based on quality
            let ratio = match quality {
                1..=30 => 0.3,
                31..=50 => 0.5,
                51..=70 => 0.8,
                71..=85 => 1.2,
                86..=95 => 1.8,
                _ => 2.5,
            };
            (original_size as f64 * ratio) as u64
        } else {
            // Calculate based on pixel count and quality
            let pixel_count = (width as u64) * (height as u64);
            // Base bytes per pixel at quality 85
            let base_bpp = 0.15; 
            // Quality multiplier (non-linear)
            let quality_mult = (quality as f64 / 85.0).powf(1.5);
            let estimated = (pixel_count as f64 * base_bpp * quality_mult) as u64;
            // Add some overhead for metadata
            estimated + 2048
        }
    }

    #[tauri::command]
    pub fn scan_heic_files(paths: Vec<String>) -> Result<Vec<HeicFileInfo>, String> {
        let mut all_files = Vec::new();
        let mut seen = HashSet::new();
        for path_str in paths {
            let path = PathBuf::from(&path_str);
            if !path.exists() { continue; }
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "zip" {
                        if let Ok(files) = extract_heic_from_zip(&path) {
                            for file in files {
                                if seen.insert(file.clone()) {
                                    if let Ok(info) = get_heic_info_internal(&file) {
                                        all_files.push(info);
                                    }
                                }
                            }
                        }
                    } else if is_heic_file(&path) && seen.insert(path.clone()) {
                        if let Ok(info) = get_heic_info_internal(&path) {
                            all_files.push(info);
                        }
                    }
                }
            } else if path.is_dir() {
                for entry in walkdir::WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                    let entry_path = entry.path();
                    if entry_path.is_file() && is_heic_file(entry_path) && seen.insert(entry_path.to_path_buf()) {
                        if let Ok(info) = get_heic_info_internal(entry_path) {
                            all_files.push(info);
                        }
                    }
                }
            }
        }
        Ok(all_files)
    }

    #[tauri::command]
    pub fn get_heic_info(path: String) -> Result<HeicFileInfo, String> {
        get_heic_info_internal(Path::new(&path))
    }

    /// Calculate estimated output sizes for given files and quality
    #[tauri::command]
    pub fn calculate_size_estimate(files: Vec<HeicFileInfo>, quality: u8) -> Result<SizeEstimate, String> {
        let quality = quality.clamp(1, 100);
        
        let original_total: u64 = files.iter().map(|f| f.size_bytes).sum();
        let estimated_total: u64 = files.iter()
            .map(|f| estimate_jpeg_size(f.size_bytes, quality, f.width, f.height))
            .sum();
        
        let savings_percent = if original_total > 0 {
            ((original_total as f64 - estimated_total as f64) / original_total as f64 * 100.0) as f32
        } else {
            0.0
        };

        Ok(SizeEstimate {
            original_total,
            estimated_total,
            savings_percent: savings_percent.max(-100.0).min(100.0),
        })
    }

    #[tauri::command]
    pub fn convert_and_export(files: Vec<String>, quality: u8, output_dir: String) -> Result<ConversionResult, String> {
        let quality = quality.clamp(1, 100);
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let zip_name = format!("heic2jpg_{}.zip", timestamp);
        let output_path = PathBuf::from(&output_dir).join(&zip_name);
        let zip_file = File::create(&output_path).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipWriter::new(BufWriter::new(zip_file));
        let options: FileOptions<()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);
        let mut converted_count = 0;
        let mut failed_count = 0;
        for file_path in &files {
            let path = PathBuf::from(file_path);
            let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let jpg_name = format!("{}.jpg", file_stem);
            match convert_heic_to_jpeg(&path, quality) {
                Ok(jpeg_data) => {
                    zip.start_file(&jpg_name, options).map_err(|e| e.to_string())?;
                    zip.write_all(&jpeg_data).map_err(|e| e.to_string())?;
                    converted_count += 1;
                }
                Err(e) => {
                    eprintln!("Failed to convert {}: {}", file_path, e);
                    failed_count += 1;
                }
            }
        }
        zip.finish().map_err(|e| e.to_string())?;
        let message = if failed_count > 0 {
            format!("Converted {} files, {} failed", converted_count, failed_count)
        } else {
            format!("Successfully converted {} files", converted_count)
        };
        Ok(ConversionResult {
            success: converted_count > 0,
            output_path: output_path.to_string_lossy().to_string(),
            message,
        })
    }

    #[cfg(target_os = "macos")]
    fn base64_encode(input: &[u8]) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
        for chunk in input.chunks(3) {
            let buf = match chunk.len() {
                1 => [(chunk[0] & 0xfc) >> 2, (chunk[0] & 0x03) << 4, 0, 0],
                2 => [(chunk[0] & 0xfc) >> 2, ((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xf0) >> 4), (chunk[1] & 0x0f) << 2, 0],
                3 => [(chunk[0] & 0xfc) >> 2, ((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xf0) >> 4),
                      ((chunk[1] & 0x0f) << 2) | ((chunk[2] & 0xc0) >> 6), chunk[2] & 0x3f],
                _ => unreachable!(),
            };
            result.push(CHARSET[buf[0] as usize] as char);
            result.push(CHARSET[buf[1] as usize] as char);
            result.push(if chunk.len() > 1 { CHARSET[buf[2] as usize] as char } else { '=' });
            result.push(if chunk.len() > 2 { CHARSET[buf[3] as usize] as char } else { '=' });
        }
        result
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::scan_heic_files,
            commands::get_heic_info,
            commands::calculate_size_estimate,
            commands::convert_and_export
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
