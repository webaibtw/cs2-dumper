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
| **`functions.json`** | All functions (Pattern, RVA, Match count) across 6 modules | [Raw functions.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/functions.json) |
| **`offsets.json`** | Numerical memory offsets and function RVAs | [Raw offsets.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.json) |
| **`patterns.json`** | IDA / AOB bytecode signatures with wildcards | [Raw patterns.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.json) |
| **`client_dll.json`** | All `client.dll` Source 2 classes, structs & fields | [Raw client_dll.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.json) |
| **`buttons.json`** | Input and movement button offsets | [Raw buttons.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/buttons.json) |
| **`interfaces.json`** | Source 2 registered interfaces & version strings | [Raw interfaces.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/interfaces.json) |
| **`info.json`** | Current CS2 build number and dump timestamp | [Raw info.json](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/info.json) |

### Language Headers
- **C / C++ (`.hpp`)**: [functions.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/functions.hpp) • [offsets.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.hpp) • [patterns.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.hpp) • [client_dll.hpp](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.hpp)
- **C# / .NET (`.cs`)**: [functions.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/functions.cs) • [offsets.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.cs) • [patterns.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.cs) • [client_dll.cs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.cs)
- **Rust (`.rs`)**: [functions.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/functions.rs) • [offsets.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.rs) • [patterns.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.rs) • [client_dll.rs](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.rs)
- **Zig (`.zig`)**: [functions.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/functions.zig) • [offsets.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/offsets.zig) • [patterns.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/patterns.zig) • [client_dll.zig](https://raw.githubusercontent.com/webaibtw/cs2-dumper/main/output/client_dll.zig)

---

## 🎯 Game Functions & Signatures (6 Modules)

Verified signatures, relative virtual addresses (RVA), and occurrence match count for each function across the 6 key engine modules (`client.dll`, `engine2.dll`, `scenesystem.dll`, `particles.dll`, `inputsystem.dll`, `panorama.dll`):

> 🟢 **1 match = Unique signature** (ideal for hooking and pattern scanning)  
> 🟡 **2+ matches = Ambiguous** (requires extra context or prologue bytes)

| Module | Function | Pattern Bytes (Signature) | RVA | Matches | Status |
| :--- | :--- | :--- | :--- | :---: | :---: |
| **`client.dll`** | `CreateInterface` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` | `0x19CB4E0` | 1 | 🟢 Unique |
| **`client.dll`** | `TraceShape` | `48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00` | `0xA19EF0` | 1 | 🟢 Unique |
| **`client.dll`** | `SetViewAngles` | `85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00` | `0xB80BA0` | 1 | 🟢 Unique |
| **`client.dll`** | `GetBaseEntity` | `48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36` | `0x142E300` | 1 | 🟢 Unique |
| **`client.dll`** | `GetBonePosition` | `48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF` | `0xE5DF50` | 1 | 🟢 Unique |
| **`client.dll`** | `InstallSchemaBindings` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?` | `0x19D8C50` | 1 | 🟢 Unique |
| **`client.dll`** | `BinaryProperties_GetValue` | `83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?` | `0xC31430` | 1 | 🟢 Unique |
| **`engine2.dll`** | `CreateInterface` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` | `0x409390` | 1 | 🟢 Unique |
| **`engine2.dll`** | `Source2Main` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 41 56 48 81 EC 80 00 00 00` | `0x20F8B0` | 1 | 🟢 Unique |
| **`engine2.dll`** | `InstallSchemaBindings` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?` | `0x4227E0` | 1 | 🟢 Unique |
| **`engine2.dll`** | `ExtractModuleMetadata` | `40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?` | `0x247E30` | 1 | 🟢 Unique |
| **`engine2.dll`** | `GetResourceManifests` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56` | `0x3C3B0` | 59 | 🟡 Ambigu |
| **`scenesystem.dll`** | `CreateInterface` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` | `0x4C7960` | 1 | 🟢 Unique |
| **`scenesystem.dll`** | `InstallSchemaBindings` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?` | `0x4C86D0` | 1 | 🟢 Unique |
| **`scenesystem.dll`** | `BinaryProperties_GetValue` | `83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?` | `0x161CD0` | 1 | 🟢 Unique |
| **`scenesystem.dll`** | `ExtractModuleMetadata` | `40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?` | `0x162190` | 1 | 🟢 Unique |
| **`scenesystem.dll`** | `GetResourceManifests` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56` | `0x1D620` | 77 | 🟡 Ambigu |
| **`particles.dll`** | `CreateInterface` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` | `0x4253E0` | 1 | 🟢 Unique |
| **`particles.dll`** | `InstallSchemaBindings` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?` | `0x429B50` | 1 | 🟢 Unique |
| **`particles.dll`** | `BinaryProperties_GetValue` | `83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?` | `0xD03C0` | 1 | 🟢 Unique |
| **`particles.dll`** | `ExtractModuleMetadata` | `40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?` | `0xD0810` | 1 | 🟢 Unique |
| **`particles.dll`** | `GetResourceManifests` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56` | `0x12180` | 22 | 🟡 Ambigu |
| **`inputsystem.dll`** | `CreateInterface` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` | `0x16F00` | 1 | 🟢 Unique |
| **`inputsystem.dll`** | `InstallSchemaBindings` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?` | `0x1FF40` | 1 | 🟢 Unique |
| **`inputsystem.dll`** | `BinaryProperties_GetValue` | `83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?` | `0x16270` | 1 | 🟢 Unique |
| **`inputsystem.dll`** | `ExtractModuleMetadata` | `40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?` | `0x166C0` | 1 | 🟢 Unique |
| **`inputsystem.dll`** | `GetResourceManifests` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56` | `0x163A0` | 6 | 🟡 Ambigu |
| **`panorama.dll`** | `CreateInterface` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` | `0x3DBD40` | 1 | 🟢 Unique |
| **`panorama.dll`** | `CreatePanoramaUIEngineInternal` | `48 89 5C 24 18 48 89 6C 24 20 56 57 41 55 41 56 41 57 48 83 EC 20 48 8B` | `0xA21F0` | 1 | 🟢 Unique |
| **`panorama.dll`** | `InstallSchemaBindings` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?` | `0x3F1240` | 1 | 🟢 Unique |
| **`panorama.dll`** | `ExtractModuleMetadata` | `40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?` | `0x9F540` | 1 | 🟢 Unique |
| **`panorama.dll`** | `GetResourceManifests` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56` | `0x8D180` | 65 | 🟡 Ambigu |

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
