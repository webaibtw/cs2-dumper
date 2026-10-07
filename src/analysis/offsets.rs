use std::collections::BTreeMap;

use anyhow::Result;

use log::{debug, error};

use memflow::prelude::v1::*;

use pelite::pattern;
use pelite::pattern::{Atom, save_len};
use pelite::pe64::{Pe, PeView, Rva};

use phf::{Map, phf_map};

pub type OffsetMap = BTreeMap<String, BTreeMap<String, Rva>>;

macro_rules! pattern_map {
    ($($module:ident => {
        $($name:expr => $pattern:expr $(=> $callback:expr)?),+ $(,)?
    }),+ $(,)?) => {
        $(
            mod $module {
                use super::*;

                pub(super) const PATTERNS: Map<
                    &'static str,
                    (
                        &'static [Atom],
                        Option<fn(&PeView, &mut BTreeMap<String, Rva>, Rva)>,
                    ),
                > = phf_map! {
                    $($name => ($pattern, $($callback)?)),+
                };

                pub fn offsets(view: PeView<'_>) -> BTreeMap<String, Rva> {
                    let mut map = BTreeMap::new();

                    for (&name, (pat, callback)) in &PATTERNS {
                        let mut save = vec![0; save_len(pat)];

                        if !view.scanner().finds_code(pat, &mut save) {
                            error!("outdated pattern: {}", name);

                            continue;
                        }

                        let rva = save[1];

                        map.insert(name.to_string(), rva);

                        if let Some(callback) = callback {
                            callback(&view, &mut map, rva);
                        }
                    }

                    for (name, value) in &map {
                        debug!(
                            "found \"{}\" at {:#X} ({}.dll + {:#X})",
                            name,
                            *value as u64 + view.optional_header().ImageBase,
                            stringify!($module),
                            value
                        );
                    }

                    map
                }
            }
        )+
    };
}

pattern_map! {
    client => {
        "dwCSGOInput" => pattern!("488905${'} 0f57c0 0f1105") => Some(|view, map, rva| {
            let mut save = [0; 2];

            if view.scanner().finds_code(pattern!("f2420f108428u4"), &mut save) {
                map.insert("dwViewAngles".to_string(), rva + save[1]);
            }
        }),
        "dwEntityList" => pattern!("48890d${'} e9${} cc") => None,
        "dwGameEntitySystem" => pattern!("488b1d${'} 48891d[4] 4c63b3") => None,
        "dwGameEntitySystem_highestEntityIndex" => pattern!("ff81u4 4885d2") => None,
        "dwGameRules" => pattern!("f6c1010f85${} 4c8b05${'} 4d85") => None,
        "dwGameTraceManager" => pattern!("488b05${'} f048ff00 4c8bb5") => None,
        "dwGlobalVars" => pattern!("488915${'} 488942") => None,
        "dwGlowManager" => pattern!("488b05${'} c3 cccccccccccccccc 8b41") => None,
        "dwLocalPlayerController" => pattern!("488b05${'} 4189be") => None,
        "dwPlantedC4" => pattern!("488b1d${'} 4532f6") => None,
        "dwPrediction" => pattern!("488d05${'} c3 cccccccccccccccc 405356 4154") => Some(|view, map, rva| {
            let mut save = [0; 2];

            if view.scanner().finds_code(pattern!("4c39b6u4 74? 4488be"), &mut save) {
                map.insert("dwLocalPlayerPawn".to_string(), rva + save[1]);
            }
        }),
        "dwSensitivity" => pattern!("488d0d${[8]'} 0f57c90f28f0") => Some(|_view, map, _rva| {
            map.insert("dwSensitivity_sensitivity".to_string(), 0x58);
        }),
        "dwViewMatrix" => pattern!("488d0d${'} 48c1e006") => None,
        "dwViewRender" => pattern!("488905${'} 488bc8 4885c0") => None,
        "dwWeaponC4" => pattern!("488b15${'} 488b5c24? ffc0 8905${} 488bc6 488934ea 80be") => None,
        "fnCreateInterface" => pattern!("' 4c8b0d???? 4c8bd2 4c8bd9 4d85c9 74? 498b4108") => None,
        "fnGetBaseEntity" => pattern!("' 48896c24? 574883ec? 448b49? bdffffff7f 4423cd 488bf9 418bc8 4585c0 7436") => None,
        "fnGetBonePosition" => pattern!("' 48895c24? 48897c24? 55488bec4883ec? e8???? 488d05??fdff") => None,
        "fnSetViewAngles" => pattern!("' 85d2 75? 486381???? f2410f1000") => None,
        "fnTraceShape" => pattern!("' 48895424? 48894c24? 55535657415441564157 488dac24???? b8??0000") => None,
    },
    engine2 => {
        "dwBuildNumber" => pattern!("8905${'} 488d0d${} ff15${} 488b0d") => None,
        "dwNetworkGameClient" => pattern!("48893d${'} ff87") => None,
        "dwNetworkGameClient_clientTickCount" => pattern!("8b81u4 c3 cccccccccccccccccc 8b81${} c3 cccccccccccccccccc 83b9") => None,
        "dwNetworkGameClient_deltaTick" => pattern!("4c8db7u4 4c897c24") => None,
        "dwNetworkGameClient_isBackgroundMap" => pattern!("0fb681u4 c3 cccccccccccccccc 0fb681${} c3 cccccccccccccccc 4883ec") => None,
        "dwNetworkGameClient_localPlayer" => pattern!("428b94d3u4 5b 49ffe3 32c0 5b c3 cccccccccccccccc 4053") => None,
        "dwNetworkGameClient_maxClients" => pattern!("8b81u4 c3????????? 8b81[4] c3????????? 8b81") => None,
        "dwNetworkGameClient_serverTickCount" => pattern!("8b81u4 c3 cccccccccccccccccc 83b9") => None,
        "dwNetworkGameClient_signOnState" => pattern!("448b81u4 488d0d") => None,
        "dwWindowHeight" => pattern!("8b05${'} 8903") => None,
        "dwWindowWidth" => pattern!("8b05${'} 8907") => None,
        "fnCreateInterface" => pattern!("' 4c8b0d???? 4c8bd2 4c8bd9 4d85c9 74? 498b4108") => None,
    },
    input_system => {
        "dwInputSystem" => pattern!("488905${'} 33c0") => None,
    },
    matchmaking => {
        "dwGameTypes" => pattern!("488d0d${'} ff90") => None,
    },
    soundsystem => {
        "dwSoundSystem" => pattern!("488d0d${'} e8${} 488b0d${} [3] 4c8b82") => None,
        "dwSoundSystem_engineViewData" => pattern!("0f1147u1 0f104f10 0f114f7c") => None,
    },
}

pub fn offsets<P: Process + MemoryView>(process: &mut P) -> Result<OffsetMap> {
    let mut map = BTreeMap::new();

    let modules: [(&str, fn(PeView) -> BTreeMap<String, u32>); 5] = [
        ("client.dll", client::offsets),
        ("engine2.dll", engine2::offsets),
        ("inputsystem.dll", input_system::offsets),
        ("matchmaking.dll", matchmaking::offsets),
        ("soundsystem.dll", soundsystem::offsets),
    ];

    for (module_name, offsets) in &modules {
        let module = process.module_by_name(module_name)?;

        let buf = process
            .read_raw(module.base, module.size as _)
            .data_part()?;

        let view = PeView::from_bytes(&buf)?;

        map.insert(module_name.to_string(), offsets(view));
    }

    Ok(map)
}

pub fn offsets_from_view(module_name: &str, view: PeView) -> Option<BTreeMap<String, Rva>> {
    match module_name {
        "client.dll" => Some(client::offsets(view)),
        "engine2.dll" => Some(engine2::offsets(view)),
        "inputsystem.dll" => Some(input_system::offsets(view)),
        "matchmaking.dll" => Some(matchmaking::offsets(view)),
        "soundsystem.dll" => Some(soundsystem::offsets(view)),
        _ => None,
    }
}

pub fn pe_file_to_view(bytes: &[u8]) -> Result<Vec<u8>> {
    let file = pelite::pe64::PeFile::from_bytes(bytes)?;
    let optional_header = file.optional_header();
    let sizeof_image = optional_header.SizeOfImage as usize;
    let sizeof_headers = optional_header.SizeOfHeaders as usize;

    let mut view = vec![0u8; sizeof_image];

    let header_copy_len = sizeof_headers.min(bytes.len());
    view[..header_copy_len].copy_from_slice(&bytes[..header_copy_len]);

    for section in file.section_headers() {
        let va = section.VirtualAddress as usize;
        let raw_ptr = section.PointerToRawData as usize;
        let raw_size = section.SizeOfRawData as usize;

        if raw_ptr >= bytes.len() || va >= sizeof_image {
            continue;
        }

        let copy_len = raw_size
            .min(bytes.len().saturating_sub(raw_ptr))
            .min(sizeof_image.saturating_sub(va));

        if copy_len > 0 {
            view[va..va + copy_len].copy_from_slice(&bytes[raw_ptr..raw_ptr + copy_len]);
        }
    }

    Ok(view)
}

pub fn offsets_from_dll_bytes(module_name: &str, bytes: &[u8]) -> Result<BTreeMap<String, Rva>> {
    let view_buf = pe_file_to_view(bytes)?;
    let view = PeView::from_bytes(&view_buf)?;
    offsets_from_view(module_name, view)
        .ok_or_else(|| anyhow::anyhow!("unknown module: {}", module_name))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Once;

    use serde_json::Value;

    use simplelog::*;

    use super::*;

    #[test]
    fn test_offline_client_dll() {
        let client_path = if std::path::Path::new(r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive\game\csgo\bin\win64\client.dll").exists() {
            r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive\game\csgo\bin\win64\client.dll"
        } else {
            r"cs2_dlls\game\csgo\bin\win64\client.dll"
        };

        let engine_path = if std::path::Path::new(r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive\game\bin\win64\engine2.dll").exists() {
            r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive\game\bin\win64\engine2.dll"
        } else {
            r"cs2_dlls\game\bin\win64\engine2.dll"
        };

        if let Ok(bytes) = std::fs::read(client_path) {
            let res = offsets_from_dll_bytes("client.dll", &bytes);
            assert!(res.is_ok());
            let map = res.unwrap();
            println!("Offline client.dll offsets count: {}", map.len());
            for (k, v) in &map {
                println!("  {}: 0x{:X}", k, v);
            }
            assert!(map.contains_key("dwEntityList"));
            assert!(map.contains_key("dwGameTraceManager"));
            assert!(map.contains_key("fnCreateInterface"));
            assert!(map.contains_key("fnGetBaseEntity"));
            assert!(map.contains_key("fnGetBonePosition"));
            assert!(map.contains_key("fnSetViewAngles"));
            assert!(map.contains_key("fnTraceShape"));
        }

        if let Ok(bytes) = std::fs::read(engine_path) {
            let res = offsets_from_dll_bytes("engine2.dll", &bytes);
            assert!(res.is_ok());
            let map = res.unwrap();
            println!("Offline engine2.dll offsets count: {}", map.len());
            for (k, v) in &map {
                println!("  {}: 0x{:X}", k, v);
            }
            assert!(map.contains_key("dwBuildNumber"));
            assert!(map.contains_key("fnCreateInterface"));
        }
    }

    #[test]
    fn build_number() -> Result<()> {
        let mut process = setup()?;

        let engine_base = process.module_by_name("engine2.dll")?.base;

        let offset = read_offset("engine2.dll", "dwBuildNumber").unwrap();

        let build_number: u32 = process.read(engine_base + offset).data_part()?;

        debug!("build number: {}", build_number);

        Ok(())
    }

    #[test]
    fn global_vars() -> Result<()> {
        let mut process = setup()?;

        let client_base = process.module_by_name("client.dll")?.base;

        let offset = read_offset("client.dll", "dwGlobalVars").unwrap();

        let global_vars: u64 = process.read(client_base + offset).data_part()?;

        let map_name_addr = process
            .read_addr64((global_vars + 0x180).into())
            .data_part()?;

        let map_name = process.read_utf8(map_name_addr, 128).data_part()?;

        debug!("[global vars] map name: \"{}\"", map_name);

        Ok(())
    }

    #[test]
    fn local_controller() -> Result<()> {
        let mut process = setup()?;

        let client_base = process.module_by_name("client.dll")?.base;

        let local_controller_offset = read_offset("client.dll", "dwLocalPlayerController").unwrap();

        let player_name_offset =
            read_class_field("client.dll", "CBasePlayerController", "m_iszPlayerName").unwrap();

        let local_controller: u64 = process
            .read(client_base + local_controller_offset)
            .data_part()?;

        let player_name = process
            .read_utf8((local_controller + player_name_offset).into(), 128)
            .data_part()?;

        debug!("[local controller] name: \"{}\"", player_name);

        Ok(())
    }

    #[test]
    fn local_pawn() -> Result<()> {
        #[derive(Pod)]
        #[repr(C)]
        struct Vector3D {
            x: f32,
            y: f32,
            z: f32,
        }

        let mut process = setup()?;

        let client_base = process.module_by_name("client.dll")?.base;

        let local_player_pawn_offset = read_offset("client.dll", "dwLocalPlayerPawn").unwrap();

        let game_scene_node_offset =
            read_class_field("client.dll", "C_BaseEntity", "m_pGameSceneNode").unwrap();

        let origin_offset =
            read_class_field("client.dll", "CGameSceneNode", "m_vecAbsOrigin").unwrap();

        let local_player_pawn: u64 = process
            .read(client_base + local_player_pawn_offset)
            .data_part()?;

        let game_scene_node: u64 = process
            .read((local_player_pawn + game_scene_node_offset).into())
            .data_part()?;

        let origin: Vector3D = process
            .read((game_scene_node + origin_offset).into())
            .data_part()?;

        debug!(
            "[local pawn] origin: {:.2}, y: {:.2}, z: {:.2}",
            origin.x, origin.y, origin.z
        );

        Ok(())
    }

    #[test]
    fn window_size() -> Result<()> {
        let mut process = setup()?;

        let engine_base = process.module_by_name("engine2.dll")?.base;

        let window_width_offset = read_offset("engine2.dll", "dwWindowWidth").unwrap();
        let window_height_offset = read_offset("engine2.dll", "dwWindowHeight").unwrap();

        let window_width: u32 = process
            .read(engine_base + window_width_offset)
            .data_part()?;

        let window_height: u32 = process
            .read(engine_base + window_height_offset)
            .data_part()?;

        debug!("window size: {}x{}", window_width, window_height);

        Ok(())
    }

    fn setup() -> Result<IntoProcessInstanceArcBox<'static>> {
        static LOGGER: Once = Once::new();

        LOGGER.call_once(|| {
            SimpleLogger::init(LevelFilter::Trace, Config::default()).ok();
        });

        let os = memflow_native::create_os(&OsArgs::default(), LibArc::default())?;

        let process = os.into_process_by_name("cs2.exe")?;

        Ok(process)
    }

    fn read_class_field(module_name: &str, class_name: &str, field_name: &str) -> Option<u64> {
        let content =
            fs::read_to_string(format!("output/{}.json", module_name.replace(".", "_"))).ok()?;

        let value: Value = serde_json::from_str(&content).ok()?;

        value
            .get(module_name)?
            .get("classes")?
            .get(class_name)?
            .get("fields")?
            .get(field_name)?
            .as_u64()
    }

    fn read_offset(module_name: &str, offset_name: &str) -> Option<u64> {
        let content = fs::read_to_string("output/offsets.json").ok()?;
        let value: Value = serde_json::from_str(&content).ok()?;

        let offset = value.get(module_name)?.get(offset_name)?;

        offset.as_u64()
    }
}
