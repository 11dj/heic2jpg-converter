# Building Windows App

## ✅ Windows Now Fully Supported!

The Windows build now includes **full HEIC conversion support** using PowerShell with .NET Imaging!

---

## How Windows HEIC Conversion Works

The app uses **PowerShell with System.Drawing** for HEIC conversion:

```powershell
# Get image dimensions
[System.Drawing.Image]::FromFile('image.heic')

# Convert to JPEG with quality
$encoder = [System.Drawing.Imaging.ImageCodecInfo]::GetImageEncoders() | 
           Where-Object { $_.FormatDescription -eq 'JPEG' }
$img.Save('output.jpg', $encoder, $encoderParams)
```

---

## Prerequisites for Windows Users

### Required: HEIF Image Extensions

Windows users need to install **HEIF Image Extensions** from Microsoft Store:

1. Open **Microsoft Store**
2. Search for **"HEIF Image Extensions"**
3. Click **Install** (it's free)

This is required because Windows doesn't natively support HEIC files without this extension.

### Alternative: CopyTrans HEIC

If the Microsoft Store extension doesn't work:
- Download: https://www.copytrans.net/copytransheic/
- Free for personal use

---

## GitHub Actions (Recommended - FREE)

The GitHub Actions workflow automatically builds for macOS and Windows.

### Steps:

1. **Push your code to GitHub**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git branch -M main
   git remote add origin https://github.com/11dj/heic2jpg-converter.git
   git push -u origin main
   ```

2. **Go to GitHub repository → Actions → "Build macOS and Windows" → Run workflow**

3. **Download the built Windows installer from Releases page**

---

## Build on Windows Machine

If you have a Windows PC:

### 1. Install Prerequisites on Windows:

- **Node.js 20+**: https://nodejs.org/
- **Rust**: https://rustup.rs/
- **Visual Studio Build Tools**: https://aka.ms/buildtools
  - Select "Desktop development with C++"

### 2. Build:

```powershell
git clone https://github.com/11dj/heic2jpg-converter.git
cd heic2jpg-converter
npm install
npm run tauri build
```

### 3. Find installer at:
```
src-tauri\target\release\bundle\msi\HEIC2JPG Converter_1.1.0_x64_en-US.msi
```

---

## Windows-Specific Notes

### What Works

| Feature | Status | Notes |
|---------|--------|-------|
| File scanning | ✅ Works | Detects HEIC files |
| Size estimation | ✅ Works | Based on file size and quality |
| HEIC conversion | ✅ Works | Via PowerShell + .NET |
| Thumbnails | ✅ Works | Generated via PowerShell |
| ZIP export | ✅ Works | Standard ZIP creation |

### Windows Output Files

After successful build:

| File | Description |
|------|-------------|
| `*.msi` | Windows Installer (Recommended) |
| `*.exe` (NSIS) | Setup executable |

---

## Troubleshooting

### "Failed to convert: PowerShell image conversion failed"

**Cause:** HEIF Image Extensions not installed

**Solution:**
1. Open Microsoft Store
2. Search "HEIF Image Extensions"
3. Install it
4. Restart the app

### Error: "Unable to find WebView2Loader.dll"

**Solution:**
- Ensure WebView2 Runtime is installed (usually included in Windows 10/11)
- Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

### Error: "Microsoft Visual Studio is required"

**Solution:**
- Install Visual Studio Build Tools with "Desktop development with C++" workload

---

## Quick Summary

| Method | Difficulty | Cost | Time |
|--------|------------|------|------|
| GitHub Actions | Easy | Free | ~10 min |
| Windows PC | Easy | Free | ~5 min |

**Recommendation:** Use GitHub Actions for automated builds!

---

## Technical Details

### Why PowerShell?

- Windows has no native HEIC command-line tool
- libheif (C library) is complex to bundle
- PowerShell + .NET Imaging is built into Windows
- Uses Windows' own HEIC codec (via HEIF Image Extensions)

### Quality Setting

JPEG quality is controlled via .NET's EncoderParameter:
```powershell
$encoderParams.Param[0] = New-Object System.Drawing.Imaging.EncoderParameter(
    [System.Drawing.Imaging.Encoder]::Quality, 
    [long]$quality  # 1-100
)
```
