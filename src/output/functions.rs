use std::fmt::{self, Write};

use heck::{AsPascalCase, AsSnakeCase};

use super::{CodeWriter, Formatter, slugify, zig_ident};
use crate::analysis::FunctionMap;

impl CodeWriter for FunctionMap {
    fn write_cs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.block("namespace CS2Dumper.Functions", false, |fmt| {
            for (module_name, functions) in self {
                writeln!(fmt, "// Module: {}", module_name)?;

                fmt.block(
                    &format!("public static class {}", AsPascalCase(slugify(module_name))),
                    false,
                    |fmt| {
                        for (name, info) in functions {
                            writeln!(
                                fmt,
                                "// Pattern: {} (Matches: {})",
                                info.pattern, info.matches
                            )?;
                            writeln!(
                                fmt,
                                "public const nint {} = {};",
                                name, info.rva_hex
                            )?;
                        }

                        Ok(())
                    },
                )?;
            }

            Ok(())
        })
    }

    fn write_hpp(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#pragma once\n")?;
        writeln!(fmt, "#include <cstddef>\n")?;

        fmt.block("namespace cs2_dumper", false, |fmt| {
            fmt.block("namespace functions", false, |fmt| {
                for (module_name, functions) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!("namespace {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for (name, info) in functions {
                                writeln!(
                                    fmt,
                                    "// Pattern: {} (Matches: {})",
                                    info.pattern, info.matches
                                )?;
                                writeln!(
                                    fmt,
                                    "constexpr std::ptrdiff_t {} = {};",
                                    name, info.rva_hex
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }

    fn write_json(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str(&serde_json::to_string_pretty(self).unwrap())
    }

    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#![allow(non_upper_case_globals, unused)]\n")?;

        fmt.block("pub mod cs2_dumper", false, |fmt| {
            fmt.block("pub mod functions", false, |fmt| {
                for (module_name, functions) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!("pub mod {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for (name, info) in functions {
                                writeln!(
                                    fmt,
                                    "// Pattern: {} (Matches: {})",
                                    info.pattern, info.matches
                                )?;
                                writeln!(
                                    fmt,
                                    "pub const {}: usize = {};",
                                    name, info.rva_hex
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }

    fn write_zig(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.block("pub const cs2_dumper = struct", true, |fmt| {
            fmt.block("pub const functions = struct", true, |fmt| {
                for (module_name, functions) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    let module_name = zig_ident(&AsSnakeCase(slugify(module_name)).to_string());

                    fmt.block(
                        &format!("pub const {} = struct", module_name),
                        true,
                        |fmt| {
                            for (name, info) in functions {
                                writeln!(
                                    fmt,
                                    "// Pattern: {} (Matches: {})",
                                    info.pattern, info.matches
                                )?;
                                writeln!(
                                    fmt,
                                    "pub const {}: usize = {};",
                                    zig_ident(name),
                                    info.rva_hex
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }
}
