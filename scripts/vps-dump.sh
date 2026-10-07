#!/usr/bin/env bash
set -e

APP_ID=730
DEPOT_ID=2347771
WORKDIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DLL_DIR="$WORKDIR/cs2_dlls"
FILELIST="$WORKDIR/scripts/filelist.txt"
BIN_DIR="$WORKDIR/bin_depot"

echo "[+] Preparing directories..."
mkdir -p "$DLL_DIR" "$BIN_DIR"

if ! command -v DepotDownloader &> /dev/null && [ ! -f "$BIN_DIR/DepotDownloader" ]; then
    echo "[+] Downloading DepotDownloader for Linux..."
    LATEST_TAG=$(curl -s https://api.github.com/repos/SteamRE/DepotDownloader/releases/latest | grep '"tag_name":' | head -n 1 | cut -d '"' -f 4)
    ARCH=$(uname -m)
    case "$ARCH" in
        x86_64) ASSET_ARCH="linux-x64" ;;
        aarch64) ASSET_ARCH="linux-arm64" ;;
        *) echo "[-] Unsupported arch: $ARCH"; exit 1 ;;
    esac
    DOWNLOAD_URL="https://github.com/SteamRE/DepotDownloader/releases/download/${LATEST_TAG}/DepotDownloader-${ASSET_ARCH}.zip"
    curl -sL -o /tmp/depotdownloader.zip "$DOWNLOAD_URL"
    unzip -q -o /tmp/depotdownloader.zip -d "$BIN_DIR"
    chmod +x "$BIN_DIR/DepotDownloader"
    DEPOT_CMD="$BIN_DIR/DepotDownloader"
elif [ -f "$BIN_DIR/DepotDownloader" ]; then
    DEPOT_CMD="$BIN_DIR/DepotDownloader"
else
    DEPOT_CMD="DepotDownloader"
fi

echo "[+] Fetching CS2 DLLs directly from Valve Steam CDN (App: $APP_ID, Depot: $DEPOT_ID)..."
"$DEPOT_CMD" -app "$APP_ID" -depot "$DEPOT_ID" -filelist "$FILELIST" -dir "$DLL_DIR"

echo "[+] Running cs2-dumper offline on downloaded DLL files..."
cd "$WORKDIR"
cargo run --release -- --offline -d "$DLL_DIR" -vv

echo "[+] Offsets & patterns dumped successfully to: $WORKDIR/output"
