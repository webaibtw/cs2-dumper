use std::collections::BTreeMap;

use anyhow::Result;
use memflow::prelude::v1::*;
use pelite::pattern::{self, save_len};
use pelite::pe64::{Pe, PeView, Rva};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionInfo {
    pub pattern: String,
    pub rva: u32,
    pub rva_hex: String,
    pub matches: usize,
}

pub type FunctionMap = BTreeMap<String, BTreeMap<String, FunctionInfo>>;

/// Definitions of function signatures to scan per module.
pub const FUNCTION_PATTERNS: &[(&str, &[(&str, &str)])] = &[
    (
        "client.dll",
        &[
            (
                "CreateInterface",
                "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08",
            ),
            (
                "TraceShape",
                "48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? B8 ? ? 00 00",
            ),
            (
                "SetViewAngles",
                "85 D2 75 ? 48 63 81 ? ? ? ? F2 41 0F 10 00",
            ),
            (
                "GetBaseEntity",
                "48 89 6C 24 ? 57 48 83 EC ? 44 8B 49 ? BD FF FF FF 7F 44 23 CD 48 8B F9 41 8B C8 45 85 C0 74 36",
            ),
            (
                "GetBonePosition",
                "48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? E8 ? ? ? ? 48 8D 05 ? ? FD FF",
            ),
            (
                "InstallSchemaBindings",
                "40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?",
            ),
            (
                "BinaryProperties_GetValue",
                "83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?",
            ),
        ],
    ),
    (
        "engine2.dll",
        &[
            (
                "CreateInterface",
                "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08",
            ),
            (
                "Source2Main",
                "48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 41 56 48 81 EC 80 00 00 00",
            ),
            (
                "InstallSchemaBindings",
                "40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?",
            ),
            (
                "ExtractModuleMetadata",
                "40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?",
            ),
            (
                "GetResourceManifests",
                "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56",
            ),
        ],
    ),
    (
        "scenesystem.dll",
        &[
            (
                "CreateInterface",
                "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08",
            ),
            (
                "InstallSchemaBindings",
                "40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?",
            ),
            (
                "BinaryProperties_GetValue",
                "83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?",
            ),
            (
                "ExtractModuleMetadata",
                "40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?",
            ),
            (
                "GetResourceManifests",
                "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56",
            ),
        ],
    ),
    (
        "particles.dll",
        &[
            (
                "CreateInterface",
                "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08",
            ),
            (
                "InstallSchemaBindings",
                "40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?",
            ),
            (
                "BinaryProperties_GetValue",
                "83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?",
            ),
            (
                "ExtractModuleMetadata",
                "40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?",
            ),
            (
                "GetResourceManifests",
                "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56",
            ),
        ],
    ),
    (
        "inputsystem.dll",
        &[
            (
                "CreateInterface",
                "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08",
            ),
            (
                "InstallSchemaBindings",
                "40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?",
            ),
            (
                "BinaryProperties_GetValue",
                "83 F9 07 0F 87 ? ? ? ? 48 63 C1 4C 8D 05 ? ? ? ? 41 8B 8C 80 ?",
            ),
            (
                "ExtractModuleMetadata",
                "40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?",
            ),
            (
                "GetResourceManifests",
                "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56",
            ),
        ],
    ),
    (
        "panorama.dll",
        &[
            (
                "CreateInterface",
                "4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08",
            ),
            (
                "CreatePanoramaUIEngineInternal",
                "48 89 5C 24 18 48 89 6C 24 20 56 57 41 55 41 56 41 57 48 83 EC 20 48 8B",
            ),
            (
                "InstallSchemaBindings",
                "40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ?",
            ),
            (
                "ExtractModuleMetadata",
                "40 56 48 83 EC 20 65 48 8B 04 25 58 00 00 00 48 8B F1 8B 15 ? ? ? ?",
            ),
            (
                "GetResourceManifests",
                "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56",
            ),
        ],
    ),
];

/// Scans a PE view for defined function signatures and counts matches.
pub fn scan_functions_from_view(module_name: &str, view: PeView<'_>) -> BTreeMap<String, FunctionInfo> {
    let mut map = BTreeMap::new();

    let module_patterns = FUNCTION_PATTERNS
        .iter()
        .find(|(name, _)| *name == module_name)
        .map(|(_, pats)| *pats);

    if let Some(patterns) = module_patterns {
        for (fn_name, pat_str) in patterns {
            if let Ok(pat) = pattern::parse(pat_str) {
                let mut save = vec![0; save_len(&pat)];
                let mut matches = view.scanner().matches_code(&pat);
                let mut count = 0;
                let mut first_rva: Rva = 0;

                while matches.next(&mut save) {
                    if count == 0 {
                        first_rva = save[0];
                    }
                    count += 1;
                }

                if count > 0 {
                    map.insert(
                        fn_name.to_string(),
                        FunctionInfo {
                            pattern: pat_str.to_string(),
                            rva: first_rva,
                            rva_hex: format!("0x{:X}", first_rva),
                            matches: count,
                        },
                    );
                }
            }
        }
    }

    map
}

/// Fallback / live analysis using Memflow process view
pub fn functions<P: Process + MemoryView>(process: &mut P) -> Result<FunctionMap> {
    let mut map = BTreeMap::new();

    for (module_name, _) in FUNCTION_PATTERNS {
        if let Ok(module) = process.module_by_name(module_name) {
            if let Ok(buf) = process.read_raw(module.base, module.size as _).data_part() {
                if let Ok(view) = PeView::from_bytes(&buf) {
                    let fns = scan_functions_from_view(module_name, view);
                    if !fns.is_empty() {
                        map.insert(module_name.to_string(), fns);
                    }
                }
            }
        }
    }

    Ok(map)
}
