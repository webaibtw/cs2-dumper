// Generated using https://github.com/a2x/cs2-dumper
// 2026-10-07 10:20:05.262624599 UTC

#![allow(non_upper_case_globals, unused)]

pub mod cs2_dumper {
    pub mod patterns {
        // Module: client.dll
        pub mod client_dll {
            pub const dwCSGOInput: &str = "48 89 05 ? ? ? ? 0F 57 C0 0F 11 05";
            pub const dwEntityList: &str = "48 89 0D ? ? ? ? E9 ? ? ? ? CC";
            pub const dwGameEntitySystem: &str = "48 8B 1D ? ? ? ? 48 89 1D ? ? ? ? 4C 63 B3";
            pub const dwGameEntitySystem_highestEntityIndex: &str = "FF 81 ? ? ? ? 48 85 D2";
            pub const dwGameRules: &str = "F6 C1 01 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 4D 85";
            pub const dwGameTraceManager: &str = "48 8B 05 ? ? ? ? F0 48 FF 00 4C 8B B5";
            pub const dwGlobalVars: &str = "48 89 15 ? ? ? ? 48 89 42";
            pub const dwGlowManager: &str = "48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 8B 41";
            pub const dwLocalPlayerController: &str = "48 8B 05 ? ? ? ? 41 89 BE";
            pub const dwLocalPlayerPawn: &str = "4C 39 B6 ? ? ? ? 74 ? 44 88 BE";
            pub const dwPlantedC4: &str = "48 8B 1D ? ? ? ? 45 32 F6";
            pub const dwPrediction: &str = "48 8D 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 40 53 56 41 54";
            pub const dwSensitivity: &str = "48 8D 0D ? ? ? ? 0F 57 C9 0F 28 F0";
            pub const dwViewAngles: &str = "F2 42 0F 10 84 28 ? ? ? ?";
            pub const dwViewMatrix: &str = "48 8D 0D ? ? ? ? 48 C1 E0 06";
            pub const dwViewRender: &str = "48 89 05 ? ? ? ? 48 8B C8 48 85 C0";
            pub const dwWeaponC4: &str = "48 8B 15 ? ? ? ? 48 8B 5C 24 ? FF C0 89 05 ? ? ? ? 48 8B C6 48 89 34 EA 80 BE";
            pub const fnCreateInterface: &str = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
            pub const fnGetBaseEntity: &str = "48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36";
            pub const fnGetBonePosition: &str = "48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF";
            pub const fnSetViewAngles: &str = "85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00";
            pub const fnTraceShape: &str = "48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00";
        }
        // Module: engine2.dll
        pub mod engine2_dll {
            pub const dwBuildNumber: &str = "89 05 ? ? ? ? 48 8D 0D ? ? ? ? FF 15 ? ? ? ? 48 8B 0D";
            pub const dwNetworkGameClient: &str = "48 89 3D ? ? ? ? FF 87";
            pub const dwNetworkGameClient_clientTickCount: &str = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
            pub const dwNetworkGameClient_deltaTick: &str = "4C 8D B7 ? ? ? ? 4C 89 7C 24";
            pub const dwNetworkGameClient_isBackgroundMap: &str = "0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 48 83 EC";
            pub const dwNetworkGameClient_localPlayer: &str = "42 8B 94 D3 ? ? ? ? 5B 49 FF E3 32 C0 5B C3 CC CC CC CC CC CC CC CC 40 53";
            pub const dwNetworkGameClient_maxClients: &str = "8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81";
            pub const dwNetworkGameClient_serverTickCount: &str = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
            pub const dwNetworkGameClient_signOnState: &str = "44 8B 81 ? ? ? ? 48 8D 0D";
            pub const dwWindowHeight: &str = "8B 05 ? ? ? ? 89 03";
            pub const dwWindowWidth: &str = "8B 05 ? ? ? ? 89 07";
            pub const fnCreateInterface: &str = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
        }
        // Module: inputsystem.dll
        pub mod inputsystem_dll {
            pub const dwInputSystem: &str = "48 89 05 ? ? ? ? 33 C0";
        }
        // Module: matchmaking.dll
        pub mod matchmaking_dll {
            pub const dwGameTypes: &str = "48 8D 0D ? ? ? ? FF 90";
        }
        // Module: soundsystem.dll
        pub mod soundsystem_dll {
            pub const dwSoundSystem: &str = "48 8D 0D ? ? ? ? E8 ? ? ? ? 48 8B 0D ? ? ? ? ? ? ? 4C 8B 82";
            pub const dwSoundSystem_engineViewData: &str = "0F 11 47 ? 0F 10 4F 10 0F 11 4F 7C";
        }
    }
}
