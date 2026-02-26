# Building Windows App from macOS

## Option 1: GitHub Actions (Recommended - FREE)

I've created a GitHub Actions workflow at `.github/workflows/build-windows.yml`.

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

2. **Go to GitHub repository → Actions → "Build Windows App" → Run workflow**

3. **Download the built Windows installer from the artifacts**

---

## Option 2: Build on Windows Machine

If you have access to a Windows PC:

1. **Install prerequisites on Windows:**
   - Node.js 20+: https://nodejs.org/
   - Rust: https://rustup.rs/
   - Visual Studio Build Tools: https://aka.ms/buildtools
     - Select "Desktop development with C++"

2. **Clone and build:**
   ```powershell
   git clone https://github.com/11dj/heic2jpg-converter.git
   cd heic2jpg-converter
   npm install
   npm run tauri build
   ```

3. **Find installer at:**
   ```
   src-tauri\target\release\bundle\msi\HEIC2JPG Converter_1.0.0_x64_en-US.msi
   ```

---

## Option 3: Use Docker (Advanced)

Create a `Dockerfile.windows`:

```dockerfile
FROM mcr.microsoft.com/windows/servercore:ltsc2022

# Install chocolatey
RUN powershell -Command "
    Set-ExecutionPolicy Bypass -Scope Process -Force;
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
    iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
"

# Install dependencies
RUN choco install -y nodejs rust visualstudio2022buildtools

WORKDIR /app
COPY . .

RUN npm install
RUN cargo install tauri-cli
RUN npm run tauri build
```

Build:
```bash
docker build -f Dockerfile.windows -t heic2jpg-builder .
docker create --name extract heic2jpg-builder
docker cp extract:/app/src-tauri/target/release/bundle ./windows-build
docker rm extract
```

---

## Option 4: Use Cross-Compilation Tool (Experimental)

Install `cargo-xwin` for cross-compiling from macOS to Windows:

```bash
# Install cargo-xwin
cargo install cargo-xwin

# Install llvm for llvm-rc
brew install llvm

# Set environment variables
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"

# Build (experimental, may not work)
npm run tauri build -- --target x86_64-pc-windows-msvc
```

**Note:** This method often fails due to Windows resource compiler requirements.

---

## Recommended: GitHub Actions

The GitHub Actions method is the most reliable and FREE way to build Windows apps from macOS.

### Setting up GitHub Actions:

1. Create a GitHub repository at https://github.com/new
2. Name it `heic2jpg-converter`
3. Push your code there
4. Go to the "Actions" tab
5. Click "Build Windows App"
6. Click "Run workflow"
7. Wait ~10 minutes
8. Download the MSI from the artifacts section

---

## Windows-Specific Notes

### HEIC Support on Windows

The Windows build requires HEIC codec support:

1. **Install HEIF Image Extensions from Microsoft Store:**
   - Open Microsoft Store
   - Search for "HEIF Image Extensions"
   - Install (it's free)

2. **Alternative: CopyTrans HEIC for Windows**
   - Download: https://www.copytrans.net/copytransheic/
   - Free for personal use

Without these, the app can scan HEIC files but cannot convert them.

### Windows Output Files

After successful build:

| File | Description |
|------|-------------|
| `*.msi` | Windows Installer (Recommended) |
| `*.exe` (NSIS) | Setup executable |
| `*.exe` (portable) | Standalone executable |

---

## Quick Summary

| Method | Difficulty | Cost | Time |
|--------|------------|------|------|
| GitHub Actions | Easy | Free | ~10 min |
| Windows PC | Easy | Free | ~5 min |
| Docker | Hard | Free | ~30 min |
| Cross-compile | Very Hard | Free | Often fails |

**Recommendation:** Use GitHub Actions for automated builds!
