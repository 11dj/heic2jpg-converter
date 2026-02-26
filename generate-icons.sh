#!/bin/bash

# Icon Generation Script for HEIC2JPG Converter
# Usage: ./generate-icons.sh <path-to-source-image>

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <path-to-source-image>"
    echo "Example: $0 ~/Downloads/my-logo.png"
    exit 1
fi

SOURCE_IMAGE="$1"
ICONS_DIR="src-tauri/icons"

if [ ! -f "$SOURCE_IMAGE" ]; then
    echo "Error: Source image not found: $SOURCE_IMAGE"
    exit 1
fi

echo "Generating icons from: $SOURCE_IMAGE"
echo "Output directory: $ICONS_DIR"

# Create icons directory if it doesn't exist
mkdir -p "$ICONS_DIR"

# Generate PNG icons for various sizes
convert "$SOURCE_IMAGE" -resize 32x32! "$ICONS_DIR/32x32.png"
convert "$SOURCE_IMAGE" -resize 128x128! "$ICONS_DIR/128x128.png"
convert "$SOURCE_IMAGE" -resize 256x256! "$ICONS_DIR/128x128@2x.png"
convert "$SOURCE_IMAGE" -resize 107x107! "$ICONS_DIR/Square107x107Logo.png"
convert "$SOURCE_IMAGE" -resize 142x142! "$ICONS_DIR/Square142x142Logo.png"
convert "$SOURCE_IMAGE" -resize 150x150! "$ICONS_DIR/Square150x150Logo.png"
convert "$SOURCE_IMAGE" -resize 284x284! "$ICONS_DIR/Square284x284Logo.png"
convert "$SOURCE_IMAGE" -resize 30x30! "$ICONS_DIR/Square30x30Logo.png"
convert "$SOURCE_IMAGE" -resize 310x310! "$ICONS_DIR/Square310x310Logo.png"
convert "$SOURCE_IMAGE" -resize 44x44! "$ICONS_DIR/Square44x44Logo.png"
convert "$SOURCE_IMAGE" -resize 71x71! "$ICONS_DIR/Square71x71Logo.png"
convert "$SOURCE_IMAGE" -resize 89x89! "$ICONS_DIR/Square89x89Logo.png"
convert "$SOURCE_IMAGE" -resize 50x50! "$ICONS_DIR/StoreLogo.png"
convert "$SOURCE_IMAGE" -resize 512x512! "$ICONS_DIR/icon.png"

# Generate macOS .icns file
mkdir -p iconset.iconset
convert "$SOURCE_IMAGE" -resize 16x16! iconset.iconset/icon_16x16.png
convert "$SOURCE_IMAGE" -resize 32x32! iconset.iconset/icon_16x16@2x.png
convert "$SOURCE_IMAGE" -resize 32x32! iconset.iconset/icon_32x32.png
convert "$SOURCE_IMAGE" -resize 64x64! iconset.iconset/icon_32x32@2x.png
convert "$SOURCE_IMAGE" -resize 128x128! iconset.iconset/icon_128x128.png
convert "$SOURCE_IMAGE" -resize 256x256! iconset.iconset/icon_128x128@2x.png
convert "$SOURCE_IMAGE" -resize 256x256! iconset.iconset/icon_256x256.png
convert "$SOURCE_IMAGE" -resize 512x512! iconset.iconset/icon_256x256@2x.png
convert "$SOURCE_IMAGE" -resize 512x512! iconset.iconset/icon_512x512.png
convert "$SOURCE_IMAGE" -resize 1024x1024! iconset.iconset/icon_512x512@2x.png

iconutil -c icns iconset.iconset -o "$ICONS_DIR/icon.icns"
rm -rf iconset.iconset

# Generate Windows .ico file
convert "$SOURCE_IMAGE" -resize 16x16! -resize 32x32! -resize 48x48! -resize 256x256! "$ICONS_DIR/icon.ico"

echo "✅ Icons generated successfully!"
echo ""
echo "Generated files:"
ls -la "$ICONS_DIR/"
