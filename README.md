<div align="center">

<img src="https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/730/header.jpg" alt="Counter-Strike 2 Banner" width="680" style="border-radius: 8px;"/>

# Counter-Strike 2 • Offsets, Patterns & Schemas

[![Auto Dump](https://github.com/webaibtw/cs2-dumper/actions/workflows/auto_dump.yml/badge.svg)](https://github.com/webaibtw/cs2-dumper/actions/workflows/auto_dump.yml)
[![CS2 Build](https://img.shields.io/badge/CS2%20Build-14189-F79F1A?logo=steam)](./output/info.json)
[![Auto Update](https://img.shields.io/badge/Update%20Frequency-Every%206%20Hours-00b894)](.github/workflows/auto_dump.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

**Automated public tracking repository for Counter-Strike 2 offsets, AOB patterns, game functions, interfaces, and Source 2 schemas.**  
Updated automatically on every CS2 game patch via cloud CI/CD directly from Valve's official CDN.

[**Browse Output Directory (Files)**](./output) • [**Raw API Access**](#-raw-api--direct-links) • [**Game Functions**](#-game-functions--signatures)

---

</div>

## 🌐 Overview

This repository is a **public, ready-to-use data source** for developers, modders, and reverse engineers working with Counter-Strike 2. 

Instead of setting up reverse engineering tools or memory scrapers locally, you can consume automatically generated and verified data files directly from this repository in **C++**, **C#**, **Rust**, **Zig**, and **JSON**.

### 🌟 Key Highlights
- **Direct Data Access**: All offsets and schemas are refreshed automatically whenever Valve pushes a CS2 update.
- **Pattern Signatures (AOB)**: Clean IDA-style signatures (`48 89 05 ? ? ? ?...`) provided for long-term stability across updates.
- **Game Functions**: Key engine functions included (`TraceShape`, `SetViewAngles`, `GetBaseEntity`, `GetBonePosition`, `CreateInterface`, `GameTraceManager`).
- **5 Formats Supported**: `.hpp`, `.cs`, `.rs`, `.zig`, and `.json` files ready for drop-in inclusion.
- **100% Automated**: Runs via headless GitHub Actions pipelines extracting directly from game binaries without running the game client.

---

## 📥 Raw API & Direct Links

You can fetch the latest raw data directly into your programs over HTTP:

| File / Component | Description | Raw Direct Link |
| :--- | :--- | :--- |
| **`offsets.json`** | Numerical memory offsets and function RVAs | [Raw offsets.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.json) |
| **`patterns.json`** | IDA / AOB bytecode signatures with wildcards | [Raw patterns.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.json) |
| **`client_dll.json`** | All `client.dll` Source 2 classes, structs & fields | [Raw client_dll.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.json) |
| **`buttons.json`** | Input and movement button offsets | [Raw buttons.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/buttons.json) |
| **`interfaces.json`** | Source 2 registered interfaces & version strings | [Raw interfaces.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/interfaces.json) |
| **`info.json`** | Current CS2 build number and dump timestamp | [Raw info.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/info.json) |

### Language Headers
- **C / C++ (`.hpp`)**: [offsets.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.hpp) • [patterns.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.hpp) • [client_dll.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.hpp)
- **C# / .NET (`.cs`)**: [offsets.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.cs) • [patterns.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.cs) • [client_dll.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.cs)
- **Rust (`.rs`)**: [offsets.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.rs) • [patterns.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.rs) • [client_dll.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.rs)
- **Zig (`.zig`)**: [offsets.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.zig) • [patterns.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.zig) • [client_dll.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.zig)

---

## 🎯 Game Functions & Signatures

In addition to standard entity pointers, this repository provides verified signatures and offsets for key game functions:

| Function | Module | Description | IDA Pattern Signature |
| :--- | :--- | :--- | :--- |
| **`fnTraceShape`** | `client.dll` | World raytracing & line of sight checks | `48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00` |
| **`dwGameTraceManager`**| `client.dll` | Singleton instance pointer for trace operations | `48 8B 05 ? ? ? ? F0 48 FF 00 4C 8B B5` |
| **`fnSetViewAngles`** | `client.dll` | Angle modification method on `CCSGOInput` | `85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00` |
| **`fnGetBaseEntity`** | `client.dll` | Fast entity pointer lookup from entity index | `48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36` |
| **`fnGetBonePosition`**| `client.dll` | Skeleton bone position resolver | `48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF` |
| **`fnCreateInterface`** | `client.dll` | Source 2 interface factory function | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` |

---

## 🛠️ CLI Tool & Manual Execution

While data is published automatically, the underlying CLI tool is open-source and can be executed locally on Windows and Linux:

```bash
# Clone the repository
git clone https://github.com/webaibtw/cs2-dumper.git
cd cs2-dumper

# Run automated VPS dump (downloads DLLs and dumps everything without game running)
./scripts/vps-dump.sh        # On Linux
./scripts/vps-dump.ps1       # On Windows

# Or scan an existing game folder offline
cargo run --release -- --offline -d "/path/to/Counter-Strike Global Offensive"
```

### CLI Arguments
- `-d, --game-dir <path>`: Path to directory containing CS2 DLLs for offline scanning.
- `--offline`: Force offline scanning mode without attaching to process memory.
- `-f, --file-types <types>`: Output file formats (`cs`, `hpp`, `json`, `rs`, `zig`).
- `-o, --output <dir>`: Output folder destination (default: `output`).

---

## 🤝 Credits & Acknowledgments

- **[a2x](https://github.com/a2x)**: Original creator of [cs2-dumper](https://github.com/a2x/cs2-dumper) and Source 2 Schema System parser.
- **[webaibtw](https://github.com/webaibtw)**: Public tracking repository maintainer, offline DLL scanner, game function signatures (`TraceShape`, `SetViewAngles`, `GetBaseEntity`, `GetBonePosition`, `CreateInterface`), IDA pattern generator, and automated CI/CD pipeline.
- **[memflow](https://github.com/memflow/memflow)**: Physical memory introspection framework.
- **[pelite](https://github.com/CasualX/pelite)**: High-performance PE binary parser and pattern scanner.
- **[SteamRE / DepotDownloader](https://github.com/SteamRE/DepotDownloader)**: Steam CDN depot downloading tool.

---

## 📜 License

Licensed under the [MIT License](./LICENSE).
