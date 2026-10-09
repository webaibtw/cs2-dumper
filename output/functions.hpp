// Generated using https://github.com/a2x/cs2-dumper
// 2026-10-09 05:22:17.944697100 UTC

#pragma once

#include <cstddef>

namespace cs2_dumper {
    namespace functions {
        // Module: client.dll
        namespace client_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            constexpr std::ptrdiff_t BinaryProperties_GetValue = 0xC30770;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            constexpr std::ptrdiff_t CreateInterface = 0x19C6770;
            // Pattern: 48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36 (Matches: 1)
            constexpr std::ptrdiff_t GetBaseEntity = 0x14296F0;
            // Pattern: 48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF (Matches: 1)
            constexpr std::ptrdiff_t GetBonePosition = 0xE5DA50;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t InstallSchemaBindings = 0x19D3EE0;
            // Pattern: 85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00 (Matches: 1)
            constexpr std::ptrdiff_t SetViewAngles = 0xB7FEE0;
            // Pattern: 48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00 (Matches: 1)
            constexpr std::ptrdiff_t TraceShape = 0xA19140;
        }
        // Module: engine2.dll
        namespace engine2_dll {
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            constexpr std::ptrdiff_t CreateInterface = 0x409390;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t ExtractModuleMetadata = 0x247E30;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 59)
            constexpr std::ptrdiff_t GetResourceManifests = 0x3C3B0;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t InstallSchemaBindings = 0x4227E0;
            // Pattern: 48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 41 56 48 81 EC 80 00 00 00 (Matches: 1)
            constexpr std::ptrdiff_t Source2Main = 0x20F8B0;
        }
        // Module: inputsystem.dll
        namespace inputsystem_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            constexpr std::ptrdiff_t BinaryProperties_GetValue = 0x16270;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            constexpr std::ptrdiff_t CreateInterface = 0x16F00;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t ExtractModuleMetadata = 0x166C0;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 6)
            constexpr std::ptrdiff_t GetResourceManifests = 0x163A0;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t InstallSchemaBindings = 0x1FF40;
        }
        // Module: panorama.dll
        namespace panorama_dll {
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            constexpr std::ptrdiff_t CreateInterface = 0x3DBD40;
            // Pattern: 48 89 5C 24 18 48 89 6C 24 20 56 57 41 55 41 56 41 57 48 83 EC 20 48 8B (Matches: 1)
            constexpr std::ptrdiff_t CreatePanoramaUIEngineInternal = 0xA21F0;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t ExtractModuleMetadata = 0x9F540;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 65)
            constexpr std::ptrdiff_t GetResourceManifests = 0x8D180;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t InstallSchemaBindings = 0x3F1240;
        }
        // Module: particles.dll
        namespace particles_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            constexpr std::ptrdiff_t BinaryProperties_GetValue = 0xD03C0;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            constexpr std::ptrdiff_t CreateInterface = 0x4253E0;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t ExtractModuleMetadata = 0xD0810;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 22)
            constexpr std::ptrdiff_t GetResourceManifests = 0x12180;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t InstallSchemaBindings = 0x429B50;
        }
        // Module: scenesystem.dll
        namespace scenesystem_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            constexpr std::ptrdiff_t BinaryProperties_GetValue = 0x161CD0;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            constexpr std::ptrdiff_t CreateInterface = 0x4C7960;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t ExtractModuleMetadata = 0x162190;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 77)
            constexpr std::ptrdiff_t GetResourceManifests = 0x1D620;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            constexpr std::ptrdiff_t InstallSchemaBindings = 0x4C86D0;
        }
    }
}
