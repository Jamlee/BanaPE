# Create a minimal valid ICO file
$icoHeader = [byte[]]@(
    0x00, 0x00, # Reserved
    0x01, 0x00, # Icon type (1 = icon)
    0x01, 0x00, # Number of images (1)
    # Image 1 directory entry
    0x10,       # Width (16 pixels)
    0x10,       # Height (16 pixels)
    0x00,       # Color count (0 = no palette)
    0x00,       # Reserved
    0x01, 0x00, # Color planes (1)
    0x20, 0x00, # Bits per pixel (32)
    0x68, 0x04, 0x00, 0x00, # Size of image data (1128 bytes)
    0x16, 0x00, 0x00, 0x00  # Offset to image data (22 bytes)
)

# Create minimal BMP data for 16x16 icon with 32-bit color
$bmpHeader = [byte[]]@(
    0x28, 0x00, 0x00, 0x00, # BITMAPINFOHEADER size (40 bytes)
    0x20, 0x00, 0x00, 0x00, # Width (32 = 16x2 for XOR+AND masks)
    0x10, 0x00, 0x00, 0x00, # Height (16)
    0x01, 0x00,             # Planes (1)
    0x20, 0x00,             # Bits per pixel (32)
    0x00, 0x00, 0x00, 0x00, # Compression (0 = none)
    0x00, 0x04, 0x00, 0x00, # Image size (1024 bytes)
    0x00, 0x00, 0x00, 0x00, # X pixels per meter
    0x00, 0x00, 0x00, 0x00, # Y pixels per meter
    0x00, 0x00, 0x00, 0x00, # Colors used
    0x00, 0x00, 0x00, 0x00  # Important colors
)

# Pixel data (16x16 pixels, 32-bit BGRA) - simple blue square
$pixelData = New-Object byte[] (16 * 16 * 4)
for ($i = 0; $i -lt $pixelData.Length; $i += 4) {
    $pixelData[$i] = 0xFF     # Blue
    $pixelData[$i + 1] = 0x00 # Green
    $pixelData[$i + 2] = 0x00 # Red
    $pixelData[$i + 3] = 0xFF # Alpha (fully opaque)
}

# AND mask (16x16 bits, padded to 32-bit boundaries)
$andMask = New-Object byte[] (16 * 4) # All zeros (fully opaque)

# Combine all parts
$iconData = $icoHeader + $bmpHeader + $pixelData + $andMask
[System.IO.File]::WriteAllBytes("$PSScriptRoot\icon.ico", $iconData)
Write-Host "Created icon.ico ($($iconData.Length) bytes)"
