// Generated using https://github.com/a2x/cs2-dumper
// 2026-10-08 05:19:03.626154081 UTC

pub const cs2_dumper = struct {
    pub const patterns = struct {
        // Module: client.dll
        pub const client_dll = struct {
            pub const dwCSGOInput: []const u8 = "48 89 05 ? ? ? ? 0F 57 C0 0F 11 05";
            pub const dwEntityList: []const u8 = "48 89 0D ? ? ? ? E9 ? ? ? ? CC";
            pub const dwGameEntitySystem: []const u8 = "48 8B 1D ? ? ? ? 48 89 1D ? ? ? ? 4C 63 B3";
            pub const dwGameEntitySystem_highestEntityIndex: []const u8 = "FF 81 ? ? ? ? 48 85 D2";
            pub const dwGameRules: []const u8 = "F6 C1 01 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 4D 85";
            pub const dwGameTraceManager: []const u8 = "48 8B 05 ? ? ? ? F0 48 FF 00 4C 8B B5";
            pub const dwGlobalVars: []const u8 = "48 89 15 ? ? ? ? 48 89 42";
            pub const dwGlowManager: []const u8 = "48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 8B 41";
            pub const dwLocalPlayerController: []const u8 = "48 8B 05 ? ? ? ? 41 89 BE";
            pub const dwLocalPlayerPawn: []const u8 = "4C 39 B6 ? ? ? ? 74 ? 44 88 BE";
            pub const dwPlantedC4: []const u8 = "48 8B 1D ? ? ? ? 45 32 F6";
            pub const dwPrediction: []const u8 = "48 8D 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 40 53 56 41 54";
            pub const dwSensitivity: []const u8 = "48 8D 0D ? ? ? ? 0F 57 C9 0F 28 F0";
            pub const dwViewAngles: []const u8 = "F2 42 0F 10 84 28 ? ? ? ?";
            pub const dwViewMatrix: []const u8 = "48 8D 0D ? ? ? ? 48 C1 E0 06";
            pub const dwViewRender: []const u8 = "48 89 05 ? ? ? ? 48 8B C8 48 85 C0";
            pub const dwWeaponC4: []const u8 = "48 8B 15 ? ? ? ? 48 8B 5C 24 ? FF C0 89 05 ? ? ? ? 48 8B C6 48 89 34 EA 80 BE";
            pub const fnCreateInterface: []const u8 = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
            pub const fnGetBaseEntity: []const u8 = "48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36";
            pub const fnGetBonePosition: []const u8 = "48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF";
            pub const fnSetViewAngles: []const u8 = "85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00";
            pub const fnTraceShape: []const u8 = "48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00";
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const dwBuildNumber: []const u8 = "89 05 ? ? ? ? 48 8D 0D ? ? ? ? FF 15 ? ? ? ? 48 8B 0D";
            pub const dwNetworkGameClient: []const u8 = "48 89 3D ? ? ? ? FF 87";
            pub const dwNetworkGameClient_clientTickCount: []const u8 = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
            pub const dwNetworkGameClient_deltaTick: []const u8 = "4C 8D B7 ? ? ? ? 4C 89 7C 24";
            pub const dwNetworkGameClient_isBackgroundMap: []const u8 = "0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 48 83 EC";
            pub const dwNetworkGameClient_localPlayer: []const u8 = "42 8B 94 D3 ? ? ? ? 5B 49 FF E3 32 C0 5B C3 CC CC CC CC CC CC CC CC 40 53";
            pub const dwNetworkGameClient_maxClients: []const u8 = "8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81";
            pub const dwNetworkGameClient_serverTickCount: []const u8 = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
            pub const dwNetworkGameClient_signOnState: []const u8 = "44 8B 81 ? ? ? ? 48 8D 0D";
            pub const dwWindowHeight: []const u8 = "8B 05 ? ? ? ? 89 03";
            pub const dwWindowWidth: []const u8 = "8B 05 ? ? ? ? 89 07";
            pub const fnCreateInterface: []const u8 = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
        };
        // Module: inputsystem.dll
        pub const inputsystem_dll = struct {
            pub const dwInputSystem: []const u8 = "48 89 05 ? ? ? ? 33 C0";
        };
        // Module: matchmaking.dll
        pub const matchmaking_dll = struct {
            pub const dwGameTypes: []const u8 = "48 8D 0D ? ? ? ? FF 90";
        };
        // Module: soundsystem.dll
        pub const soundsystem_dll = struct {
            pub const dwSoundSystem: []const u8 = "48 8D 0D ? ? ? ? E8 ? ? ? ? 48 8B 0D ? ? ? ? ? ? ? 4C 8B 82";
            pub const dwSoundSystem_engineViewData: []const u8 = "0F 11 47 ? 0F 10 4F 10 0F 11 4F 7C";
        };
    };
};
