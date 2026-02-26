#!/usr/bin/env python3
"""
Icon Generator for HEIC2JPG Converter
Generates all required icon sizes from the logo design
"""

from PIL import Image, ImageDraw, ImageFont
import os
import sys

# Colors from the logo
BG_COLOR = (160, 71, 71)  # #a04747 - Red background
WHITE = (255, 255, 255)   # White text/border

def get_font(size):
    """Try to load a system font, fallback to default"""
    # Skip font for very small sizes
    if size < 32:
        return None
    
    font_paths = [
        "/System/Library/Fonts/Helvetica.ttc",
        "/System/Library/Fonts/HelveticaNeue.ttc",
        "/Library/Fonts/Arial.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/Windows/Fonts/arial.ttf",
    ]
    
    for path in font_paths:
        try:
            return ImageFont.truetype(path, size)
        except:
            continue
    
    return None

def draw_text_simple(draw, text, y_pos, size, fill):
    """Draw text centered at y position"""
    font_size = int(size * 0.20)
    if font_size < 8:
        return  # Too small to draw text
    
    font = get_font(font_size)
    if font is None:
        return
    
    try:
        # Calculate text width
        bbox = draw.textbbox((0, 0), text, font=font)
        text_width = bbox[2] - bbox[0]
        x = (size - text_width) // 2
        draw.text((x, y_pos), text, font=font, fill=fill)
    except:
        pass  # Skip if font rendering fails

def create_icon(size):
    """Create an icon with the HEIC -> JPG logo"""
    img = Image.new('RGBA', (size, size), BG_COLOR)
    draw = ImageDraw.Draw(img)
    
    # Calculate proportions based on size
    padding = max(2, int(size * 0.08))
    line_width = max(1, int(size * 0.04))
    bracket_length = int(size * 0.25)
    
    # Draw corner brackets
    # Top-left bracket
    draw.line([(padding, padding + bracket_length), (padding, padding), (padding + bracket_length, padding)],
              fill=WHITE, width=line_width)
    
    # Top-right bracket
    draw.line([(size - padding - bracket_length, padding), (size - padding, padding), 
               (size - padding, padding + bracket_length)],
              fill=WHITE, width=line_width)
    
    # Bottom-left bracket
    draw.line([(padding, size - padding - bracket_length), (padding, size - padding), 
               (padding + bracket_length, size - padding)],
              fill=WHITE, width=line_width)
    
    # Bottom-right bracket
    draw.line([(size - padding - bracket_length, size - padding), (size - padding, size - padding), 
               (size - padding, size - padding - bracket_length)],
              fill=WHITE, width=line_width)
    
    # Draw arrow in center (pointing down)
    arrow_size = max(3, int(size * 0.12))
    center_x = size // 2
    center_y = size // 2
    
    # Simple arrow shape pointing down
    arrow_points = [
        (center_x, center_y + arrow_size),      # Arrow tip (bottom)
        (center_x - arrow_size//2, center_y - arrow_size//3),   # Top-left
        (center_x + arrow_size//2, center_y - arrow_size//3),   # Top-right
    ]
    draw.polygon(arrow_points, fill=WHITE)
    
    # Draw HEIC text (top)
    draw_text_simple(draw, "HEIC", int(size * 0.15), size, WHITE)
    
    # Draw JPG text (bottom)
    draw_text_simple(draw, "JPG", int(size * 0.60), size, WHITE)
    
    return img

def create_icns(output_path):
    """Create macOS .icns file"""
    import tempfile
    import subprocess
    
    # Create temporary iconset directory
    with tempfile.TemporaryDirectory() as tmpdir:
        iconset_path = os.path.join(tmpdir, "icon.iconset")
        os.makedirs(iconset_path)
        
        # Generate all required sizes
        sizes = [
            (16, "icon_16x16.png"),
            (32, "icon_16x16@2x.png"),
            (32, "icon_32x32.png"),
            (64, "icon_32x32@2x.png"),
            (128, "icon_128x128.png"),
            (256, "icon_128x128@2x.png"),
            (256, "icon_256x256.png"),
            (512, "icon_256x256@2x.png"),
            (512, "icon_512x512.png"),
            (1024, "icon_512x512@2x.png"),
        ]
        
        for size, filename in sizes:
            icon = create_icon(size)
            icon.save(os.path.join(iconset_path, filename))
        
        # Use iconutil to create .icns
        subprocess.run(["iconutil", "-c", "icns", iconset_path, "-o", output_path], check=True)

def create_ico(sizes, output_path):
    """Create Windows .ico file"""
    images = []
    for size in sizes:
        img = create_icon(size)
        # Convert to RGBA for ICO
        if img.mode != 'RGBA':
            img = img.convert('RGBA')
        images.append(img)
    
    # Save as ICO with multiple sizes
    images[0].save(output_path, format='ICO', sizes=[(s, s) for s in sizes], append_images=images[1:])

def main():
    icons_dir = "src-tauri/icons"
    os.makedirs(icons_dir, exist_ok=True)
    
    print("🎨 Generating icons for HEIC2JPG Converter...")
    print(f"   Output directory: {icons_dir}")
    
    # Generate standard PNG icons
    icon_sizes = {
        "32x32.png": 32,
        "128x128.png": 128,
        "128x128@2x.png": 256,
        "Square107x107Logo.png": 107,
        "Square142x142Logo.png": 142,
        "Square150x150Logo.png": 150,
        "Square284x284Logo.png": 284,
        "Square30x30Logo.png": 30,
        "Square310x310Logo.png": 310,
        "Square44x44Logo.png": 44,
        "Square71x71Logo.png": 71,
        "Square89x89Logo.png": 89,
        "StoreLogo.png": 50,
        "icon.png": 512,
    }
    
    for filename, size in icon_sizes.items():
        filepath = os.path.join(icons_dir, filename)
        icon = create_icon(size)
        icon.save(filepath)
        print(f"   ✓ {filename} ({size}x{size})")
    
    # Generate macOS .icns
    print("   Generating macOS icon set (icon.icns)...")
    try:
        create_icns(os.path.join(icons_dir, "icon.icns"))
        print("   ✓ icon.icns")
    except Exception as e:
        print(f"   ⚠️  Could not generate icon.icns: {e}")
        # Fallback: save a 512x512 PNG
        create_icon(512).save(os.path.join(icons_dir, "icon-512.png"))
    
    # Generate Windows .ico
    print("   Generating Windows icon (icon.ico)...")
    try:
        create_ico([16, 32, 48, 256], os.path.join(icons_dir, "icon.ico"))
        print("   ✓ icon.ico")
    except Exception as e:
        print(f"   ⚠️  Could not generate icon.ico: {e}")
        # Fallback
        create_icon(256).save(os.path.join(icons_dir, "icon-256.png"))
    
    print("\n✅ All icons generated successfully!")
    print(f"   Author: 11dj (https://github.com/11dj)")
    print(f"   Version: 1.0.0")

if __name__ == "__main__":
    main()
