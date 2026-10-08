// Generated using https://github.com/a2x/cs2-dumper
// 2026-10-08 13:05:47.533790772 UTC

#![allow(non_upper_case_globals, unused)]

pub mod cs2_dumper {
    pub mod functions {
        // Module: client.dll
        pub mod client_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            pub const BinaryProperties_GetValue: usize = 0xC31430;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            pub const CreateInterface: usize = 0x19CB4E0;
            // Pattern: 48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36 (Matches: 1)
            pub const GetBaseEntity: usize = 0x142E300;
            // Pattern: 48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF (Matches: 1)
            pub const GetBonePosition: usize = 0xE5DF50;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            pub const InstallSchemaBindings: usize = 0x19D8C50;
            // Pattern: 85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00 (Matches: 1)
            pub const SetViewAngles: usize = 0xB80BA0;
            // Pattern: 48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00 (Matches: 1)
            pub const TraceShape: usize = 0xA19EF0;
        }
        // Module: engine2.dll
        pub mod engine2_dll {
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            pub const CreateInterface: usize = 0x409390;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            pub const ExtractModuleMetadata: usize = 0x247E30;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 59)
            pub const GetResourceManifests: usize = 0x3C3B0;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            pub const InstallSchemaBindings: usize = 0x4227E0;
            // Pattern: 48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 41 56 48 81 EC 80 00 00 00 (Matches: 1)
            pub const Source2Main: usize = 0x20F8B0;
        }
        // Module: inputsystem.dll
        pub mod inputsystem_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            pub const BinaryProperties_GetValue: usize = 0x16270;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            pub const CreateInterface: usize = 0x16F00;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            pub const ExtractModuleMetadata: usize = 0x166C0;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 6)
            pub const GetResourceManifests: usize = 0x163A0;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            pub const InstallSchemaBindings: usize = 0x1FF40;
        }
        // Module: panorama.dll
        pub mod panorama_dll {
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            pub const CreateInterface: usize = 0x3DBD40;
            // Pattern: 48 89 5C 24 18 48 89 6C 24 20 56 57 41 55 41 56 41 57 48 83 EC 20 48 8B (Matches: 1)
            pub const CreatePanoramaUIEngineInternal: usize = 0xA21F0;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            pub const ExtractModuleMetadata: usize = 0x9F540;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 65)
            pub const GetResourceManifests: usize = 0x8D180;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            pub const InstallSchemaBindings: usize = 0x3F1240;
        }
        // Module: particles.dll
        pub mod particles_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            pub const BinaryProperties_GetValue: usize = 0xD03C0;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            pub const CreateInterface: usize = 0x4253E0;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            pub const ExtractModuleMetadata: usize = 0xD0810;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 22)
            pub const GetResourceManifests: usize = 0x12180;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            pub const InstallSchemaBindings: usize = 0x429B50;
        }
        // Module: scenesystem.dll
        pub mod scenesystem_dll {
            // Pattern: 83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ? (Matches: 1)
            pub const BinaryProperties_GetValue: usize = 0x161CD0;
            // Pattern: 4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08 (Matches: 1)
            pub const CreateInterface: usize = 0x4C7960;
            // Pattern: 40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ? (Matches: 1)
            pub const ExtractModuleMetadata: usize = 0x162190;
            // Pattern: 48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 (Matches: 77)
            pub const GetResourceManifests: usize = 0x1D620;
            // Pattern: 40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? (Matches: 1)
            pub const InstallSchemaBindings: usize = 0x4C86D0;
        }
    }
}
