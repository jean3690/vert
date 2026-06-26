# Vert CLI installer — Windows PowerShell
# Usage: irm https://raw.githubusercontent.com/jean3690/vert/master/install.ps1 | iex

$ErrorActionPreference = "Stop"
$Repo = "jean3690/vert"

Write-Host " Vert CLI installer" -ForegroundColor Cyan
Write-Host ""

# ── Get latest release ──
Write-Host "Fetching latest release..." -ForegroundColor Gray
$api = "https://api.github.com/repos/$Repo/releases/latest"
try {
    $release = Invoke-RestMethod -Uri $api -Headers @{ "Accept" = "application/vnd.github+json" }
} catch {
    # Fallback: try without authentication (rate-limited)
    $release = Invoke-RestMethod -Uri $api
}
$tag = $release.tag_name
Write-Host " Latest: $tag" -ForegroundColor Green

# ── Download ──
$bin = "vert-win-x64.exe"
$url = "https://github.com/$Repo/releases/download/$tag/$bin"
$tmp = "$env:TEMP\$bin"

Write-Host "Downloading $bin..." -ForegroundColor Gray
Invoke-WebRequest -Uri $url -OutFile $tmp

# ── Install ──
$dir = "$env:LOCALAPPDATA\vert"
New-Item -ItemType Directory -Force -Path $dir | Out-Null
$dest = "$dir\vert.exe"
Move-Item -Force $tmp $dest

# ── Add to PATH ──
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$dir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$dir", "User")
    $env:Path = "$env:Path;$dir"
    Write-Host " Added to PATH" -ForegroundColor Yellow
}

Write-Host ""
Write-Host " Done! vert installed to $dest" -ForegroundColor Green
Write-Host " Try: vert --help" -ForegroundColor Gray
