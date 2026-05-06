# BanaPE Icon Generator - Multi-resolution ICO
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

# Load required assemblies
Add-Type -AssemblyName System.Drawing

function New-IconResource {
    param(
        [int]$Width,
        [int]$Height,
        [string]$Color
    )
    
    $bmp = New-Object System.Drawing.Bitmap($Width, $Height, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = 'AntiAlias'
    $g.TextRenderingHint = 'AntiAlias'
    $g.InterpolationMode = 'HighQualityBicubic'
    $g.CompositingQuality = 'HighQuality'
    
    # Background circle with gradient
    $centerX = $Width / 2
    $centerY = $Height / 2
    $radius = ($Width / 2) * 0.92
    
    # Create gradient brush (indigo)
    $rect = [System.Drawing.RectangleF]::new(0, 0, $Width, $Height)
    $brush = [System.Drawing.Drawing2D.LinearGradientBrush]::new(
        $rect,
        [System.Drawing.Color]::FromArgb(99, 102, 241),  # #6366f1 light indigo
        [System.Drawing.Color]::FromArgb(55, 48, 163),  # #3730a3 dark indigo
        45.0  # angle
    )
    $g.FillEllipse($brush, ($centerX - $radius), ($centerY - $radius), ($radius * 2), ($radius * 2))
    $brush.Dispose()
    
    # Inner ring
    $innerRadius = $radius * 0.85
    $pen = [System.Drawing.Pen]::new([System.Drawing.Color]::FromArgb(255, 255, 255, 40), [Math]::Max(1, $Width / 64))
    $g.DrawEllipse($pen, ($centerX - $innerRadius), ($centerY - $innerRadius), ($innerRadius * 2), ($innerRadius * 2))
    $pen.Dispose()
    
    # "B" letter
    $fontSize = [Math]::Round($Width * 0.52)
    $font = New-Object System.Drawing.Font("Arial Black", $fontSize, [System.Drawing.FontStyle]::Bold)
    $textBrush = [System.Drawing.SolidBrush]::new([System.Drawing.Color]::White)
    $sf = New-Object System.Drawing.StringFormat
    $sf.Alignment = 'Center'
    $sf.LineAlignment = 'Center'
    
    # Text shadow for depth at small sizes
    if ($Width -ge 32) {
        $shadowBrush = [System.Drawing.SolidBrush]::new([System.Drawing.Color]::FromArgb(60, 30, 20, 50))
        $g.DrawString("B", $font, $shadowBrush, $centerX + 1, $centerY + 1, $sf)
        $shadowBrush.Dispose()
    }
    
    $g.DrawString("B", $font, $textBrush, $centerX, $centerY, $sf)
    
    # Highlight
    if ($Width -ge 32) {
        $highlightBrush = [System.Drawing.SolidBrush]::new([System.Drawing.Color]::FromArgb(80, 255, 255, 255))
        $highlightRect = [System.Drawing.RectangleF]::new(
            $centerX - $radius * 0.6,
            $centerY - $radius * 0.7,
            $radius * 1.2,
            $radius * 0.5
        )
        $g.FillEllipse($highlightBrush, $highlightRect)
        $highlightBrush.Dispose()
    }
    
    $textBrush.Dispose()
    $sf.Dispose()
    $font.Dispose()
    $g.Dispose()
    
    return $bmp
}

function ConvertTo-IcoData {
    param([System.Drawing.Bitmap]$Bitmap)
    
    $ms = New-Object System.IO.MemoryStream
    
    # PNG data (for ICO format with PNG compression)
    $pngMs = New-Object System.IO.MemoryStream
    $Bitmap.Save($pngMs, [System.Drawing.Imaging.ImageFormat]::Png)
    $pngBytes = $pngMs.ToArray()
    $pngMs.Close()
    
    return $pngBytes
}

Write-Host "Generating BanaPE icon..." -ForegroundColor Cyan

$sizes = @(16, 24, 32, 48, 64, 128, 256)
$imageDataList = @()

foreach ($size in $sizes) {
    Write-Host "  Generating ${size}x${size}..." -ForegroundColor Gray
    $bmp = New-IconResource -Width $size -Height $size
    $data = ConvertTo-IcoData -Bitmap $bmp
    $imageDataList += @{ Size = $size; Data = $data }
    $bmp.Dispose()
}

# Build ICO file
$icoMs = New-Object System.IO.MemoryStream
$icoWriter = [System.IO.BinaryWriter]::new($icoMs)

# ICO Header: Reserved(2) + Type(2=ICON) + Count(2)
$icoWriter.Write([uint16]0)       # Reserved
$icoWriter.Write([uint16]1)       # Type: ICON
$icoWriter.Write([uint16]$imageDataList.Count)

# Calculate offset to image data
$dataOffset = 6 + (16 * $imageDataList.Count)  # header + entries

# Directory entries + image data
$currentOffset = $dataOffset

foreach ($img in $imageDataList) {
    $w = if ($img.Size -eq 256) { 0 } else { $img.Size }
    $h = if ($img.Size -eq 256) { 0 } else { $img.Size }
    
    # ICONDIRENTRY (16 bytes)
    $icoWriter.Write([byte]$w)           # Width
    $icoWriter.Write([byte]$h)           # Height  
    $icoWriter.Write([byte]0)            # Color palette
    $icoWriter.Write([byte]0)            # Reserved
    $icoWriter.Write([uint16]1)          # Color planes
    $icoWriter.Write([uint16]32)         # Bits per pixel
    $icoWriter.Write([uint32]$img.Data.Length)  # Image size
    $icoWriter.Write([uint32]$currentOffset)     # Offset to image
    
    $currentOffset += $img.Data.Length
}

# Write actual image data (PNG format within ICO)
foreach ($img in $imageDataList) {
    $icoWriter.Write($img.Data)
}

# Save ICO file
$icoBytes = $icoMs.ToArray()
[System.IO.File]::WriteAllBytes((Join-Path $PSScriptRoot "icon.ico"), $icoBytes)
$icoWriter.Close()

# Also save individual PNGs for reference
foreach ($img in $imageDataList) {
    $pngPath = Join-Path $PSScriptRoot "icon_${Size}.png"
}

Write-Host ""
Write-Host "✅ Icon generated successfully!" -ForegroundColor Green
Write-Host "   File: $(Join-Path $PSScriptRoot 'icon.ico')" -ForegroundColor White
Write-Host "   Sizes: $($sizes -join ', ')" -ForegroundColor Gray
Write-Host "   Format: ICO (PNG compressed)" -ForegroundColor Gray
