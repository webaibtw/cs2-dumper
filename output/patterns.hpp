// Generated using https://github.com/a2x/cs2-dumper
// 2026-10-08 05:19:03.626154081 UTC

#pragma once

#include <string_view>

namespace cs2_dumper {
    namespace patterns {
        // Module: client.dll
        namespace client_dll {
            constexpr std::string_view dwCSGOInput = "48 89 05 ? ? ? ? 0F 57 C0 0F 11 05";
            constexpr std::string_view dwEntityList = "48 89 0D ? ? ? ? E9 ? ? ? ? CC";
            constexpr std::string_view dwGameEntitySystem = "48 8B 1D ? ? ? ? 48 89 1D ? ? ? ? 4C 63 B3";
            constexpr std::string_view dwGameEntitySystem_highestEntityIndex = "FF 81 ? ? ? ? 48 85 D2";
            constexpr std::string_view dwGameRules = "F6 C1 01 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 4D 85";
            constexpr std::string_view dwGameTraceManager = "48 8B 05 ? ? ? ? F0 48 FF 00 4C 8B B5";
            constexpr std::string_view dwGlobalVars = "48 89 15 ? ? ? ? 48 89 42";
            constexpr std::string_view dwGlowManager = "48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 8B 41";
            constexpr std::string_view dwLocalPlayerController = "48 8B 05 ? ? ? ? 41 89 BE";
            constexpr std::string_view dwLocalPlayerPawn = "4C 39 B6 ? ? ? ? 74 ? 44 88 BE";
            constexpr std::string_view dwPlantedC4 = "48 8B 1D ? ? ? ? 45 32 F6";
            constexpr std::string_view dwPrediction = "48 8D 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 40 53 56 41 54";
            constexpr std::string_view dwSensitivity = "48 8D 0D ? ? ? ? 0F 57 C9 0F 28 F0";
            constexpr std::string_view dwViewAngles = "F2 42 0F 10 84 28 ? ? ? ?";
            constexpr std::string_view dwViewMatrix = "48 8D 0D ? ? ? ? 48 C1 E0 06";
            constexpr std::string_view dwViewRender = "48 89 05 ? ? ? ? 48 8B C8 48 85 C0";
            constexpr std::string_view dwWeaponC4 = "48 8B 15 ? ? ? ? 48 8B 5C 24 ? FF C0 89 05 ? ? ? ? 48 8B C6 48 89 34 EA 80 BE";
            constexpr std::string_view fnCreateInterface = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
            constexpr std::string_view fnGetBaseEntity = "48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36";
            constexpr std::string_view fnGetBonePosition = "48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF";
            constexpr std::string_view fnSetViewAngles = "85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00";
            constexpr std::string_view fnTraceShape = "48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00";
        }
        // Module: engine2.dll
        namespace engine2_dll {
            constexpr std::string_view dwBuildNumber = "89 05 ? ? ? ? 48 8D 0D ? ? ? ? FF 15 ? ? ? ? 48 8B 0D";
            constexpr std::string_view dwNetworkGameClient = "48 89 3D ? ? ? ? FF 87";
            constexpr std::string_view dwNetworkGameClient_clientTickCount = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
            constexpr std::string_view dwNetworkGameClient_deltaTick = "4C 8D B7 ? ? ? ? 4C 89 7C 24";
            constexpr std::string_view dwNetworkGameClient_isBackgroundMap = "0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 0F B6 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC 48 83 EC";
            constexpr std::string_view dwNetworkGameClient_localPlayer = "42 8B 94 D3 ? ? ? ? 5B 49 FF E3 32 C0 5B C3 CC CC CC CC CC CC CC CC 40 53";
            constexpr std::string_view dwNetworkGameClient_maxClients = "8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? ? 8B 81";
            constexpr std::string_view dwNetworkGameClient_serverTickCount = "8B 81 ? ? ? ? C3 CC CC CC CC CC CC CC CC CC 83 B9";
            constexpr std::string_view dwNetworkGameClient_signOnState = "44 8B 81 ? ? ? ? 48 8D 0D";
            constexpr std::string_view dwWindowHeight = "8B 05 ? ? ? ? 89 03";
            constexpr std::string_view dwWindowWidth = "8B 05 ? ? ? ? 89 07";
            constexpr std::string_view fnCreateInterface = "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08";
        }
        // Module: inputsystem.dll
        namespace inputsystem_dll {
            constexpr std::string_view dwInputSystem = "48 89 05 ? ? ? ? 33 C0";
        }
        // Module: matchmaking.dll
        namespace matchmaking_dll {
            constexpr std::string_view dwGameTypes = "48 8D 0D ? ? ? ? FF 90";
        }
        // Module: soundsystem.dll
        namespace soundsystem_dll {
            constexpr std::string_view dwSoundSystem = "48 8D 0D ? ? ? ? E8 ? ? ? ? 48 8B 0D ? ? ? ? ? ? ? 4C 8B 82";
            constexpr std::string_view dwSoundSystem_engineViewData = "0F 11 47 ? 0F 10 4F 10 0F 11 4F 7C";
        }
    }
}
