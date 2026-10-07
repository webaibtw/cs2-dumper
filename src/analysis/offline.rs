use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use log::{debug, info, warn};

use super::{AnalysisResult, OffsetMap, offsets_from_dll_bytes, pattern_map};

const TARGET_MODULES: [&str; 5] = [
    "client.dll",
    "engine2.dll",
    "inputsystem.dll",
    "matchmaking.dll",
    "soundsystem.dll",
];

/// Analyzes CS2 modules directly from DLL files on disk without needing game memory.
pub fn analyze_offline(custom_dir: Option<&Path>) -> Result<(AnalysisResult, Option<u32>)> {
    let (dll_map, build_number) = find_cs2_dlls(custom_dir)?;

    if dll_map.is_empty() {
        bail!("no Counter-Strike 2 DLLs found on disk");
    }

    info!("found {} DLLs on disk for offline analysis", dll_map.len());

    let mut offsets = OffsetMap::new();

    for (module_name, path) in &dll_map {
        debug!("analyzing {} at {:?}", module_name, path);
        let bytes = fs::read(path)?;
        match offsets_from_dll_bytes(module_name, &bytes) {
            Ok(mod_offsets) => {
                info!("found {} offsets in {}", mod_offsets.len(), module_name);
                offsets.insert(module_name.clone(), mod_offsets);
            }
            Err(err) => {
                warn!("failed to analyze {}: {}", module_name, err);
            }
        }
    }

    let patterns = pattern_map();

    let result = AnalysisResult {
        buttons: BTreeMap::new(),
        interfaces: BTreeMap::new(),
        offsets,
        patterns,
        schemas: BTreeMap::new(),
    };

    Ok((result, build_number))
}

/// Attempts to locate CS2 DLL files from a custom directory or by scanning known Steam locations.
pub fn find_cs2_dlls(custom_dir: Option<&Path>) -> Result<(HashMap<String, PathBuf>, Option<u32>)> {
    let mut dll_map = HashMap::new();
    let mut build_number = None;

    if let Some(dir) = custom_dir {
        if dir.exists() {
            collect_dlls_from_dir(dir, &mut dll_map);

            // Also check for steam.inf
            if let Some(bn) = read_build_number_from_dir(dir) {
                build_number = Some(bn);
            }
        }
    } else {
        // Auto-detect from potential CS2 directories
        let mut candidate_dirs = Vec::new();

        // 1. Current directory
        candidate_dirs.push(PathBuf::from("."));

        // 2. Discover Steam libraries
        for lib in get_steam_libraries() {
            let cs_dir = lib
                .join("steamapps")
                .join("common")
                .join("Counter-Strike Global Offensive");

            if cs_dir.exists() {
                candidate_dirs.push(cs_dir);
            }
        }

        // 3. Common fallback directories
        for fallback in [
            r"C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive",
            r"C:\Program Files\Steam\steamapps\common\Counter-Strike Global Offensive",
            r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive",
            r"E:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive",
            r"F:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive",
        ] {
            let path = PathBuf::from(fallback);
            if path.exists() && !candidate_dirs.contains(&path) {
                candidate_dirs.push(path);
            }
        }

        for dir in candidate_dirs {
            collect_dlls_from_dir(&dir, &mut dll_map);

            if build_number.is_none() {
                build_number = read_build_number_from_dir(&dir);
            }

            if TARGET_MODULES.iter().all(|name| dll_map.contains_key(*name)) {
                break;
            }
        }
    }

    Ok((dll_map, build_number))
}

fn collect_dlls_from_dir(dir: &Path, dll_map: &mut HashMap<String, PathBuf>) {
    // 1. Check known subdirectories for CS2:
    // client.dll & matchmaking.dll in game/csgo/bin/win64
    // engine2.dll, inputsystem.dll, soundsystem.dll in game/bin/win64
    let subdirs = [
        dir.to_path_buf(),
        dir.join("game").join("csgo").join("bin").join("win64"),
        dir.join("game").join("bin").join("win64"),
        dir.join("bin").join("win64"),
    ];

    for subdir in &subdirs {
        if !subdir.exists() {
            continue;
        }

        for target in TARGET_MODULES {
            if dll_map.contains_key(target) {
                continue;
            }

            let candidate = subdir.join(target);
            if candidate.is_file() {
                dll_map.insert(target.to_string(), candidate);
            }
        }
    }

    // 2. If some DLLs are still missing, do a shallow recursive walk (depth <= 4)
    if TARGET_MODULES.iter().any(|name| !dll_map.contains_key(*name)) {
        walk_dir_for_dlls(dir, dll_map, 0, 4);
    }
}

fn walk_dir_for_dlls(
    dir: &Path,
    dll_map: &mut HashMap<String, PathBuf>,
    current_depth: usize,
    max_depth: usize,
) {
    if current_depth > max_depth {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                let lower = file_name.to_lowercase();
                for target in TARGET_MODULES {
                    if lower == *target && !dll_map.contains_key(target) {
                        dll_map.insert(target.to_string(), path.clone());
                    }
                }
            }
        } else if path.is_dir() {
            walk_dir_for_dlls(&path, dll_map, current_depth + 1, max_depth);
        }
    }
}

fn read_build_number_from_dir(dir: &Path) -> Option<u32> {
    let candidate_paths = [
        dir.join("steam.inf"),
        dir.join("game").join("csgo").join("steam.inf"),
        dir.join("csgo").join("steam.inf"),
    ];

    for path in &candidate_paths {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(val) = trimmed
                    .strip_prefix("ClientVersion=")
                    .or_else(|| trimmed.strip_prefix("ServerVersion="))
                {
                    if let Ok(bn) = val.trim().parse::<u32>() {
                        return Some(bn);
                    }
                }
            }
        }
    }

    None
}

fn get_steam_libraries() -> Vec<PathBuf> {
    let mut libraries = Vec::new();

    let steam_roots = [
        PathBuf::from(r"C:\Program Files (x86)\Steam"),
        PathBuf::from(r"C:\Program Files\Steam"),
    ];

    for root in &steam_roots {
        if !root.exists() {
            continue;
        }

        libraries.push(root.clone());

        let vdf_paths = [
            root.join("steamapps").join("libraryfolders.vdf"),
            root.join("config").join("libraryfolders.vdf"),
        ];

        for vdf_path in &vdf_paths {
            if let Ok(content) = fs::read_to_string(vdf_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("\"path\"") {
                        if let Some(rest) = trimmed.strip_prefix("\"path\"") {
                            let clean_path = rest.trim().trim_matches('"').replace(r"\\", r"\");
                            let pb = PathBuf::from(clean_path);
                            if pb.exists() && !libraries.contains(&pb) {
                                libraries.push(pb);
                            }
                        }
                    }
                }
            }
        }
    }

    libraries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cs2_dlls_auto() {
        let (dlls, bn) = find_cs2_dlls(None).unwrap();
        println!("Found DLLs: {:?}, BuildNumber: {:?}", dlls, bn);
        if !dlls.is_empty() {
            assert!(dlls.contains_key("client.dll"));
        }
    }
}
