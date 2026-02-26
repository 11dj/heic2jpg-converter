# HEIC2JPG Converter v1.0.0

![Logo](src-tauri/icons/icon.png)

A modern, cross-platform desktop application for converting HEIC/HEIF images to JPEG format. Built with **Tauri** (Rust + React) for blazing-fast performance and small bundle size.

**Author:** [11dj](https://github.com/11dj)  
**Version:** 1.0.0  
**License:** MIT

## ✨ Features

- 🖼️ **Drag & Drop Support**: Drop single files, multiple files, entire folders, or ZIP archives
- 📊 **Size Estimation**: Real-time estimate of output file size based on quality settings
- 🎚️ **Quality Control**: Adjustable JPEG quality (1-100%)
- 👁️ **Preview**: Thumbnail previews of HEIC files
- 📦 **ZIP Export**: Converted files exported as timestamped ZIP archives
- 🌙 **Dark Mode**: Automatic dark mode support
- ⚡ **Fast**: Native performance with Rust backend
- 🔒 **Privacy**: All processing happens locally - no data leaves your device
- 🪟 **Full Windows Support**: HEIC conversion now works on Windows (using libheif library)

## 📋 Prerequisites

### All Platforms

- **Node.js** (v18 or higher) - [Download](https://nodejs.org/)
- **npm** (v9 or higher) - Included with Node.js

### macOS

- **Xcode Command Line Tools** (for building native modules)
  ```bash
  xcode-select --install
  ```
- **Rust** (via rustup)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **ImageMagick** (optional, for custom icon generation)
  ```bash
  brew install imagemagick
  ```

### Windows

- **Microsoft Visual Studio C++ Build Tools** - [Download](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - Required components: "Desktop development with C++"
- **Rust** (via rustup)
  ```powershell
  # Download and run rustup-init.exe from https://rustup.rs/
  ```
- **WebView2 Runtime** - Usually pre-installed on Windows 10/11

### Linux

- **Build Essentials**
  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```
- **Rust** (via rustup)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## 🚀 How to Run (Development)

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd heic2jpg-converter
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

   This will:
   - Start the Vite development server
   - Compile the Rust backend
   - Launch the application window
   - Enable hot-reload for both frontend and backend

## 🔨 How to Build

### Build for Current Platform

```bash
npm run tauri build
```

Output locations:
- **macOS**: `src-tauri/target/release/bundle/macos/HEIC2JPG Converter.app`
- **macOS DMG**: `src-tauri/target/release/bundle/dmg/HEIC2JPG Converter_0.1.0_aarch64.dmg`
- **Windows**: `src-tauri/target/release/bundle/msi/HEIC2JPG Converter_0.1.0_x64.msi`
- **Linux**: `src-tauri/target/release/bundle/deb/heic2jpg-converter_0.1.0_amd64.deb`

### Build for Specific Platform

#### Windows (from macOS/Linux)

```bash
# Install Windows target
rustup target add x86_64-pc-windows-msvc

# Build (requires cross-compilation setup)
npm run tauri build -- --target x86_64-pc-windows-msvc
```

#### macOS Universal (Intel + Apple Silicon)

```bash
# Install targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build universal binary
npm run tauri build -- --target universal-apple-darwin
```

## 🖼️ How to Change App Icons

### Option 1: Using the Python Script (Recommended)

The project includes a Python script that generates all icons from the logo design:

```bash
python3 generate-icons.py
```

This will generate all required icon sizes automatically with the HEIC2JPG branding.

### Option 2: Using ImageMagick Script

1. Prepare your logo image (recommended: 1024x1024 PNG with transparency)
2. Run the icon generation script:
   ```bash
   ./generate-icons.sh /path/to/your-logo.png
   ```
3. Rebuild the application:
   ```bash
   npm run tauri build
   ```

### Option 2: Manual Replacement

Replace the following files in `src-tauri/icons/`:

| File | Size | Platform |
|------|------|----------|
| `32x32.png` | 32×32 | Windows/Linux |
| `128x128.png` | 128×128 | Universal |
| `128x128@2x.png` | 256×256 | macOS Retina |
| `icon.icns` | Multi-size | macOS |
| `icon.ico` | Multi-size | Windows |
| `icon.png` | 512×512 | Linux |

**Generate macOS .icns:**
```bash
mkdir MyIcon.iconset
sips -z 16 16 icon.png --out MyIcon.iconset/icon_16x16.png
sips -z 32 32 icon.png --out MyIcon.iconset/icon_16x16@2x.png
sips -z 32 32 icon.png --out MyIcon.iconset/icon_32x32.png
sips -z 64 64 icon.png --out MyIcon.iconset/icon_32x32@2x.png
sips -z 128 128 icon.png --out MyIcon.iconset/icon_128x128.png
sips -z 256 256 icon.png --out MyIcon.iconset/icon_128x128@2x.png
sips -z 256 256 icon.png --out MyIcon.iconset/icon_256x256.png
sips -z 512 512 icon.png --out MyIcon.iconset/icon_256x256@2x.png
sips -z 512 512 icon.png --out MyIcon.iconset/icon_512x512.png
sips -z 1024 1024 icon.png --out MyIcon.iconset/icon_512x512@2x.png
iconutil -c icns MyIcon.iconset -o src-tauri/icons/icon.icns
rm -rf MyIcon.iconset
```

**Generate Windows .ico:**
```bash
convert icon.png -define icon:auto-resize=16,32,48,256 src-tauri/icons/icon.ico
```

## 🪟 Windows-Specific Usage Notes

### ✅ HEIC Support on Windows - FULLY WORKING!

**Great news!** Windows now has **full HEIC to JPEG conversion support** using the `libheif` library!

- **macOS**: Uses `sips` command (built-in)
- **Windows**: Uses `libheif` library (bundled with the app)
- **Linux**: Uses `libheif` library

No additional codecs or installations needed on Windows - the app includes everything!

### How It Works

The app uses the `libheif` library (the same library used by major image software):
- ✅ Statically linked on Windows
- ✅ No external dependencies required
- ✅ Fully self-contained
- ✅ Works on Windows 10 and 11

### Windows Build Troubleshooting

**Error: "Unable to find WebView2Loader.dll"**
- Ensure WebView2 Runtime is installed (usually included in Windows 10/11)
- Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

**Error: "Microsoft Visual Studio is required"**
- Install Visual Studio Build Tools with "Desktop development with C++" workload

**Error: "Could not find `protoc`"**
- Download Protocol Buffers compiler: https://github.com/protocolbuffers/protobuf/releases
- Add to PATH

## 📂 Project Structure

```
heic2jpg-converter/
├── src/                        # React Frontend
│   ├── App.tsx                # Main UI component
│   ├── App.css                # Styles
│   └── main.tsx               # Entry point
├── src-tauri/
│   ├── src/
│   │   └── lib.rs             # Rust backend (commands)
│   ├── icons/                 # App icons
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
├── generate-icons.sh          # Icon generation script
├── package.json               # Node dependencies
└── README.md                  # This file
```

## 🔧 Configuration

### Adjusting Size Estimation Formula

The size estimation algorithm is in `src-tauri/src/lib.rs`:

```rust
fn estimate_jpeg_size(original_size: u64, quality: u8, width: u32, height: u32) -> u64 {
    // Modify this formula to adjust size estimation accuracy
    // based on your specific use case
}
```

### Changing Default Quality

Edit `src/App.tsx`:

```typescript
const [quality, setQuality] = useState<number>(85);  // Change default here
```

### Changing Output ZIP Naming

Edit `src-tauri/src/lib.rs`:

```rust
let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
let zip_name = format!("heic2jpg_{}.zip", timestamp);  // Modify pattern here
```

## 🐛 Troubleshooting

### Common Issues

**Build fails with "libheif-sys" errors**
- This project uses platform-native tools instead of libheif
- Ensure you're using the latest code from this repository

**HEIC files not recognized**
- Verify file extension is `.heic` or `.heif` (case insensitive)
- Check file isn't corrupted

**Conversion fails on macOS**
- Ensure `sips` command is available: `which sips`
- Check file permissions: `ls -la /path/to/file.heic`

**High memory usage with large batches**
- Process files in smaller batches (50-100 at a time)
- Close other applications to free RAM

**Thumbnail not showing**
- Thumbnails are generated using `sips` on macOS
- Other platforms may not show thumbnails (feature limitation)

### Getting Help

1. Check Tauri documentation: https://tauri.app/
2. File an issue with:
   - Operating system and version
   - Application version
   - Steps to reproduce
   - Error messages (if any)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit changes: `git commit -am 'Add new feature'`
4. Push to branch: `git push origin feature/my-feature`
5. Submit a Pull Request

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Icons by [Lucide](https://lucide.dev/)
- UI powered by [React](https://react.dev/)

---

## 👤 Author

**11dj** - [GitHub](https://github.com/11dj)

Feel free to ⭐ star the repository if you find this project useful!

---

## Quick Reference Card

| Task | Command |
|------|---------|
| Install dependencies | `npm install` |
| Run dev mode | `npm run tauri dev` |
| Build for current platform | `npm run tauri build` |
| Generate icons | `python3 generate-icons.py` |
| Update dependencies | `npm update && cargo update` |

**Estimated Output Size Formula:**
- Quality 1-30%: ~30% of original
- Quality 31-50%: ~50% of original  
- Quality 51-70%: ~80% of original
- Quality 71-85%: ~120% of original
- Quality 86-95%: ~180% of original
- Quality 96-100%: ~250% of original

---

<p align="center">
  <sub>Built with ❤️ by <a href="https://github.com/11dj">11dj</a> | Version 1.0.0</sub>
</p>
