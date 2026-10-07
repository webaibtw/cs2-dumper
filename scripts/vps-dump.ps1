$ErrorActionPreference = "Stop"

$AppId = 730
$DepotId = 2347771
$RootDir = Split-Path -Parent $PSScriptRoot
$DllDir = Join-Path $RootDir "cs2_dlls"
$FileList = Join-Path $PSScriptRoot "filelist.txt"
$BinDir = Join-Path $RootDir "bin_depot"

Write-Host "[+] Preparing directories..."
if (-not (Test-Path $DllDir)) { New-Item -ItemType Directory -Path $DllDir | Out-Null }
if (-not (Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir | Out-Null }

$DepotExe = Join-Path $BinDir "DepotDownloader.exe"

if (-not (Test-Path $DepotExe)) {
    Write-Host "[+] Downloading DepotDownloader for Windows..."
    $Release = Invoke-RestMethod "https://api.github.com/repos/SteamRE/DepotDownloader/releases/latest"
    $Asset = $Release.assets | Where-Object { $_.name -like "*windows-x64.zip" } | Select-Object -First 1
    if (-not $Asset) { throw "Could not find DepotDownloader windows-x64 release asset" }

    $ZipPath = Join-Path $env:TEMP "DepotDownloader.zip"
    Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $ZipPath
    Expand-Archive -Path $ZipPath -DestinationPath $BinDir -Force
    Remove-Item $ZipPath -Force
}

Write-Host "[+] Fetching CS2 DLLs directly from Valve Steam CDN (App: $AppId, Depot: $DepotId)..."
& $DepotExe -app $AppId -depot $DepotId -filelist $FileList -dir $DllDir

Write-Host "[+] Running cs2-dumper offline..."
Set-Location $RootDir
cargo run --release -- --offline -d $DllDir -vv

Write-Host "[+] Offsets & patterns dumped successfully into output/ !"
