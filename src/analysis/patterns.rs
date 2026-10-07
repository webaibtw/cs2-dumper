use std::collections::BTreeMap;

use anyhow::Result;

use memflow::prelude::v1::*;

pub type PatternMap = BTreeMap<String, BTreeMap<String, String>>;

/// Converts a Pelite pattern string into an IDA-style signature.
pub fn pelite_to_ida_pattern(pat: &str) -> String {
    let mut tokens = Vec::new();
    let chars: Vec<char> = pat.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        if c == '\'' {
            // Bookmark (save cursor), takes 0 bytes in pattern
            i += 1;
            continue;
        }

        if c == '?' {
            tokens.push("?".to_string());
            i += 1;
            continue;
        }

        if c == '$' {
            // Relative 4-byte jump
            i += 1;
            if i < chars.len() && chars[i] == '{' {
                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }
                if i < chars.len() && chars[i] == '}' {
                    i += 1;
                }
            }
            tokens.extend(["?".to_string(), "?".to_string(), "?".to_string(), "?".to_string()]);
            continue;
        }

        if c == '%' {
            // Relative 1-byte jump
            i += 1;
            if i < chars.len() && chars[i] == '{' {
                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }
                if i < chars.len() && chars[i] == '}' {
                    i += 1;
                }
            }
            tokens.push("?".to_string());
            continue;
        }

        if c == '[' {
            // [n] fixed-size skip
            i += 1;
            let mut num_str = String::new();
            while i < chars.len() && chars[i] != ']' {
                num_str.push(chars[i]);
                i += 1;
            }
            if i < chars.len() && chars[i] == ']' {
                i += 1;
            }
            if let Ok(n) = num_str.parse::<usize>() {
                for _ in 0..n {
                    tokens.push("?".to_string());
                }
            }
            continue;
        }

        if (c == 'u' || c == 'i') && i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
            let size = chars[i + 1].to_digit(10).unwrap_or(0);
            i += 2;
            for _ in 0..size {
                tokens.push("?".to_string());
            }
            continue;
        }

        if c.is_ascii_hexdigit() {
            if i + 1 < chars.len() && chars[i + 1].is_ascii_hexdigit() {
                tokens.push(format!(
                    "{}{}",
                    chars[i].to_ascii_uppercase(),
                    chars[i + 1].to_ascii_uppercase()
                ));
                i += 2;
            } else {
                tokens.push(format!("?{}", chars[i].to_ascii_uppercase()));
                i += 1;
            }
            continue;
        }

        i += 1;
    }

    tokens.join(" ")
}

pub fn pattern_map() -> PatternMap {
    let raw_modules: &[(&str, &[(&str, &str)])] = &[
        (
            "client.dll",
            &[
                ("dwCSGOInput", "488905${'} 0f57c0 0f1105"),
                ("dwEntityList", "48890d${'} e9${} cc"),
                ("dwGameEntitySystem", "488b1d${'} 48891d[4] 4c63b3"),
                ("dwGameEntitySystem_highestEntityIndex", "ff81u4 4885d2"),
                ("dwGameRules", "f6c1010f85${} 4c8b05${'} 4d85"),
                ("dwGameTraceManager", "488b05${'} f048ff00 4c8bb5"),
                ("dwGlobalVars", "488915${'} 488942"),
                ("dwGlowManager", "488b05${'} c3 cccccccccccccccc 8b41"),
                ("dwLocalPlayerController", "488b05${'} 4189be"),
                ("dwLocalPlayerPawn", "4c39b6u4 74? 4488be"),
                ("dwPlantedC4", "488b1d${'} 4532f6"),
                ("dwPrediction", "488d05${'} c3 cccccccccccccccc 405356 4154"),
                ("dwSensitivity", "488d0d${[8]'} 0f57c90f28f0"),
                ("dwViewAngles", "f2420f108428u4"),
                ("dwViewMatrix", "488d0d${'} 48c1e006"),
                ("dwViewRender", "488905${'} 488bc8 4885c0"),
                ("dwWeaponC4", "488b15${'} 488b5c24? ffc0 8905${} 488bc6 488934ea 80be"),
                ("fnCreateInterface", "' 4c8b0d???? 4c8bd2 4c8bd9 4d85c9 74? 498b4108"),
                ("fnGetBaseEntity", "' 48896c24? 574883ec? 448b49? bdffffff7f 4423cd 488bf9 418bc8 4585c0 7436"),
                ("fnGetBonePosition", "' 48895c24? 48897c24? 55488bec4883ec? e8???? 488d05??fdff"),
                ("fnSetViewAngles", "' 85d2 75? 486381???? f2410f1000"),
                ("fnTraceShape", "' 48895424? 48894c24? 55535657415441564157 488dac24???? b8??0000"),
            ],
        ),
        (
            "engine2.dll",
            &[
                ("dwBuildNumber", "8905${'} 488d0d${} ff15${} 488b0d"),
                ("dwNetworkGameClient", "48893d${'} ff87"),
                ("dwNetworkGameClient_clientTickCount", "8b81u4 c3 cccccccccccccccccc 8b81${} c3 cccccccccccccccccc 83b9"),
                ("dwNetworkGameClient_deltaTick", "4c8db7u4 4c897c24"),
                ("dwNetworkGameClient_isBackgroundMap", "0fb681u4 c3 cccccccccccccccc 0fb681${} c3 cccccccccccccccc 4883ec"),
                ("dwNetworkGameClient_localPlayer", "428b94d3u4 5b 49ffe3 32c0 5b c3 cccccccccccccccc 4053"),
                ("dwNetworkGameClient_maxClients", "8b81u4 c3????????? 8b81[4] c3????????? 8b81"),
                ("dwNetworkGameClient_serverTickCount", "8b81u4 c3 cccccccccccccccccc 83b9"),
                ("dwNetworkGameClient_signOnState", "448b81u4 488d0d"),
                ("dwWindowHeight", "8b05${'} 8903"),
                ("dwWindowWidth", "8b05${'} 8907"),
                ("fnCreateInterface", "' 4c8b0d???? 4c8bd2 4c8bd9 4d85c9 74? 498b4108"),
            ],
        ),
        (
            "inputsystem.dll",
            &[
                ("dwInputSystem", "488905${'} 33c0"),
            ],
        ),
        (
            "matchmaking.dll",
            &[
                ("dwGameTypes", "488d0d${'} ff90"),
            ],
        ),
        (
            "soundsystem.dll",
            &[
                ("dwSoundSystem", "488d0d${'} e8${} 488b0d${} [3] 4c8b82"),
                ("dwSoundSystem_engineViewData", "0f1147u1 0f104f10 0f114f7c"),
            ],
        ),
    ];

    let mut map = BTreeMap::new();

    for (module_name, patterns) in raw_modules {
        let mut module_map = BTreeMap::new();

        for (name, pat) in *patterns {
            module_map.insert(name.to_string(), pelite_to_ida_pattern(pat));
        }

        map.insert(module_name.to_string(), module_map);
    }

    map
}

pub fn patterns<P: Process + MemoryView>(_process: &mut P) -> Result<PatternMap> {
    Ok(pattern_map())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pelite_to_ida() {
        assert_eq!(
            pelite_to_ida_pattern("488905${'} 0f57c0 0f1105"),
            "48 89 05 ? ? ? ? 0F 57 C0 0F 11 05"
        );
        assert_eq!(
            pelite_to_ida_pattern("ff81u4 4885d2"),
            "FF 81 ? ? ? ? 48 85 D2"
        );
        assert_eq!(
            pelite_to_ida_pattern("0f1147u1 0f104f10 0f114f7c"),
            "0F 11 47 ? 0F 10 4F 10 0F 11 4F 7C"
        );
        assert_eq!(
            pelite_to_ida_pattern("488d0d${[8]'} 0f57c90f28f0"),
            "48 8D 0D ? ? ? ? 0F 57 C9 0F 28 F0"
        );
    }
}
