pub use buttons::*;
pub use functions::*;
pub use interfaces::*;
pub use offline::*;
pub use offsets::*;
pub use patterns::*;
pub use schemas::*;

use std::any::type_name;

use anyhow::Result;

use log::{error, info};

use memflow::prelude::v1::*;

mod buttons;
mod functions;
mod interfaces;
mod offline;
mod offsets;
mod patterns;
mod schemas;

#[derive(Debug)]
pub struct AnalysisResult {
    pub buttons: ButtonMap,
    pub functions: FunctionMap,
    pub interfaces: InterfaceMap,
    pub offsets: OffsetMap,
    pub patterns: PatternMap,
    pub schemas: SchemaMap,
}

pub fn analyze_all<P: Process + MemoryView>(process: &mut P) -> Result<AnalysisResult> {
    let buttons = analyze(process, buttons);

    info!("found {} buttons", buttons.len());

    let interfaces = analyze(process, interfaces);

    info!(
        "found {} interfaces across {} modules",
        interfaces
            .iter()
            .map(|(_, ifaces)| ifaces.len())
            .sum::<usize>(),
        interfaces.len()
    );

    let offsets = analyze(process, offsets);

    info!(
        "found {} offsets across {} modules",
        offsets
            .iter()
            .map(|(_, offsets)| offsets.len())
            .sum::<usize>(),
        offsets.len()
    );

    let patterns = analyze(process, patterns);

    info!(
        "found {} patterns across {} modules",
        patterns
            .iter()
            .map(|(_, pats)| pats.len())
            .sum::<usize>(),
        patterns.len()
    );

    let functions = analyze(process, functions);

    info!(
        "found {} functions across {} modules",
        functions
            .iter()
            .map(|(_, fns)| fns.len())
            .sum::<usize>(),
        functions.len()
    );

    let schemas = analyze(process, schemas);

    let (class_count, enum_count) =
        schemas
            .values()
            .fold((0, 0), |(classes, enums), (class_vec, enum_vec)| {
                (classes + class_vec.len(), enums + enum_vec.len())
            });

    info!(
        "found {} classes and {} enums across {} modules",
        class_count,
        enum_count,
        schemas.len()
    );

    Ok(AnalysisResult {
        buttons,
        functions,
        interfaces,
        offsets,
        patterns,
        schemas,
    })
}

fn analyze<P, F, T>(process: &mut P, f: F) -> T
where
    P: Process + MemoryView,
    F: FnOnce(&mut P) -> Result<T>,
    T: Default,
{
    let name = type_name::<F>();

    match f(process) {
        Ok(result) => result,
        Err(err) => {
            error!("failed to read {}: {}", name, err);

            T::default()
        }
    }
}
