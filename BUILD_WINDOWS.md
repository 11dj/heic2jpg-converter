# Building Windows App

## ✅ Windows Now Fully Supported!

The Windows build now includes **full HEIC conversion support** using the `libheif` library!

---

## GitHub Actions (Recommended - FREE)

The GitHub Actions workflow at `.github/workflows/build-all-platforms.yml` automatically builds for all platforms including Windows with HEIC support.

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

2. **Go to GitHub repository → Actions → "Build All Platforms" → Run workflow**

3. **Download the built Windows installer from:**
   - The `builds/windows/` folder in your repository
   - Or the Actions artifacts

---

## Build on Windows Machine

If you have a Windows PC:

### 1. Install Prerequisites on Windows:

- **Node.js 20+**: https://nodejs.org/
- **Rust**: https://rustup.rs/
- **Visual Studio Build Tools**: https://aka.ms/buildtools
  - Select "Desktop development with C++"
- **vcpkg** (for libheif):
  ```powershell
  git clone https://github.com/Microsoft/vcpkg.git
  cd vcpkg
  .\bootstrap-vcpkg.bat
  .\vcpkg install libheif:x64-windows-static
  ```

### 2. Build:

```powershell
git clone https://github.com/11dj/heic2jpg-converter.git
cd heic2jpg-converter
npm install
$env:VCPKG_ROOT = "C:\path\to\vcpkg"
$env:PKG_CONFIG_PATH = "$env:VCPKG_ROOT\installed\x64-windows-static\lib\pkgconfig"
npm run tauri build
```

### 3. Find installer at:
```
src-tauri\target\release\bundle\msi\HEIC2JPG Converter_1.0.0_x64_en-US.msi
```

---

## Windows-Specific Notes

### HEIC Support is Built-In! 🎉

The Windows app now includes:
- ✅ Full HEIC to JPEG conversion
- ✅ No additional codecs needed
- ✅ Works on Windows 10 and 11

### How It Works

The app uses the `libheif` library (the same library used by major image software) which is:
- Statically linked on Windows
- No external dependencies required
- Fully self-contained

### Windows Output Files

After successful build:

| File | Description |
|------|-------------|
| `*.msi` | Windows Installer (Recommended) |
| `*.exe` (NSIS) | Setup executable |
| `*.exe` (portable) | Standalone executable |

---

## Quick Summary

| Method | Difficulty | Cost | Time | HEIC Support |
|--------|------------|------|------|--------------|
| GitHub Actions | Easy | Free | ~15 min | ✅ Yes |
| Windows PC | Medium | Free | ~10 min | ✅ Yes |

**Recommendation:** Use GitHub Actions for automated builds!

---

## Troubleshooting

### Error: "libheif not found"

Make sure vcpkg is properly installed and the environment variables are set:
```powershell
$env:VCPKG_ROOT = "C:\path\to\vcpkg"
$env:PKG_CONFIG_PATH = "$env:VCPKG_ROOT\installed\x64-windows-static\lib\pkgconfig"
```

### Error: "Could not find `protoc`"

Download Protocol Buffers compiler: https://github.com/protocolbuffers/protobuf/releases
Add to PATH.

### Build succeeds but app doesn't convert HEIC

This should not happen with the current version. If it does:
1. Check the app logs
2. Ensure libheif was properly linked
3. Try rebuilding with verbose output: `npm run tauri build -- --verbose`
