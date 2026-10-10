// Generated using https://github.com/a2x/cs2-dumper
// 2026-10-10 21:22:35.846160404 UTC

namespace CS2Dumper.Patterns {
    // Module: client.dll
    public static class ClientDll {
        public const string dwCSGOInput = "48 89 05 ? ? ? ? 0F 57 C0 0F 11 05";
        public const string dwEntityList = "48 89 0D ? ? ? ? E9 ? ? ? ? CC";
        public const string dwGameEntitySystem = "48 8B 1D ? ? ? ? 48 89 1D ? ? ? ? 4C 63 B3";
        public const string dwGameEntitySystem_highestEntityIndex = "FF 81 ? ? ? ? 48 85 D2";
        public const string dwGameRules = "F6 C1 01 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 4D 85";
        public const string dwGameTraceManager = "48 8B 05 ? ? ? ? F0 48 FF 00 4C 8B B5";
        public const string dwGlobalVars = "48 89 15 ? ? ? ? 48 89 42";
        public const string dwGlowManager = "48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 8B 41";
        public const string dwLocalPlayerController = "48 8B 05 ? ? ? ? 41 89 BE";
        public const string dwLocalPlayerPawn = "4C 39 B6 ? ? ? ? 74 ? 44 88 BE";
        public const string dwPlantedC4 = "48 8B 1D ? ? ? ? 45 32 F6";
        public const string dwPrediction = "48 8D 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 40 53 56 41 54";
        public const string dwSensitivity = "48 8D 0D ? ? ? ? 0F 57 C9 0F 28 F0";
        public const string dwViewAngles = "F2 42 0F 10 84 28 ? ? ? ?";
        public const string dwViewMatrix = "48 8D 0D ? ? ? ? 48 C1 E0 06";
        public const string dwViewRender = "48 89 05 ? ? ? ? 48 8B C8 48 85 C0";
        public const string dwWeaponC4 = "48 8B 15 ? ? ? ? 48 8B 5C 24 ? FF C0 89 05 ? ? ? ? 48 8B C6 48 89 34 EA 80 BE";
        public const string fnCreateInterface = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
        public const string fnGetBaseEntity = "48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36";
        public const string fnGetBonePosition = "48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF";
        public const string fnSetViewAngles = "85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00";
        public const string fnTraceShape = "48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00";
    }
    // Module: engine2.dll
    public static class Engine2Dll {
        public const string dwBuildNumber = "89 05 ? ? ? ? 48 8D 0D ? ? ? ? FF 15 ? ? ? ? 48 8B 0D";
        public const string dwNetworkGameClient = "48 89 3D ? ? ? ? FF 87";
        public const string dwNetworkGameClient_clientTickCount = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
        public const string dwNetworkGameClient_deltaTick = "4C 8D B7 ? ? ? ? 4C 89 7C 24";
        public const string dwNetworkGameClient_isBackgroundMap = "0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 48 83 EC";
        public const string dwNetworkGameClient_localPlayer = "42 8B 94 D3 ? ? ? ? 5B 49 FF E3 32 C0 5B C3 CC CC CC CC CC CC CC CC 40 53";
        public const string dwNetworkGameClient_maxClients = "8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81";
        public const string dwNetworkGameClient_serverTickCount = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
        public const string dwNetworkGameClient_signOnState = "44 8B 81 ? ? ? ? 48 8D 0D";
        public const string dwWindowHeight = "8B 05 ? ? ? ? 89 03";
        public const string dwWindowWidth = "8B 05 ? ? ? ? 89 07";
        public const string fnCreateInterface = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
    }
    // Module: inputsystem.dll
    public static class InputsystemDll {
        public const string dwInputSystem = "48 89 05 ? ? ? ? 33 C0";
    }
    // Module: matchmaking.dll
    public static class MatchmakingDll {
        public const string dwGameTypes = "48 8D 0D ? ? ? ? FF 90";
    }
    // Module: soundsystem.dll
    public static class SoundsystemDll {
        public const string dwSoundSystem = "48 8D 0D ? ? ? ? E8 ? ? ? ? 48 8B 0D ? ? ? ? ? ? ? 4C 8B 82";
        public const string dwSoundSystem_engineViewData = "0F 11 47 ? 0F 10 4F 10 0F 11 4F 7C";
    }
}
