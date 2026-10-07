use std::fmt::{self, Write};

use heck::{AsPascalCase, AsSnakeCase};

use super::{CodeWriter, Formatter, PatternMap, slugify, zig_ident};

impl CodeWriter for PatternMap {
    fn write_cs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.block("namespace CS2Dumper.Patterns", false, |fmt| {
            for (module_name, patterns) in self {
                writeln!(fmt, "// Module: {}", module_name)?;

                fmt.block(
                    &format!("public static class {}", AsPascalCase(slugify(module_name))),
                    false,
                    |fmt| {
                        for (name, value) in patterns {
                            writeln!(fmt, "public const string {} = \"{}\";", name, value)?;
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
        writeln!(fmt, "#include <string_view>\n")?;

        fmt.block("namespace cs2_dumper", false, |fmt| {
            fmt.block("namespace patterns", false, |fmt| {
                for (module_name, patterns) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!("namespace {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for (name, value) in patterns {
                                writeln!(
                                    fmt,
                                    "constexpr std::string_view {} = \"{}\";",
                                    name, value
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
            fmt.block("pub mod patterns", false, |fmt| {
                for (module_name, patterns) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!("pub mod {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for (name, value) in patterns {
                                let mut name = name.clone();

                                if name == "use" {
                                    name = format!("r#{}", name);
                                }

                                writeln!(fmt, "pub const {}: &str = \"{}\";", name, value)?;
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
            fmt.block("pub const patterns = struct", true, |fmt| {
                for (module_name, patterns) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    let module_name = zig_ident(&AsSnakeCase(slugify(module_name)).to_string());

                    fmt.block(
                        &format!("pub const {} = struct", module_name),
                        true,
                        |fmt| {
                            for (name, value) in patterns {
                                writeln!(
                                    fmt,
                                    "pub const {}: []const u8 = \"{}\";",
                                    zig_ident(name),
                                    value
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::pattern_map;

    #[test]
    fn test_pattern_output_generation() {
        let patterns = pattern_map();

        for file_type in ["cs", "hpp", "json", "rs", "zig"] {
            let mut out = String::new();
            let mut fmt = Formatter::new(&mut out, 4);

            let res = match file_type {
                "cs" => patterns.write_cs(&mut fmt),
                "hpp" => patterns.write_hpp(&mut fmt),
                "json" => patterns.write_json(&mut fmt),
                "rs" => patterns.write_rs(&mut fmt),
                "zig" => patterns.write_zig(&mut fmt),
                _ => unreachable!(),
            };

            assert!(res.is_ok());
            assert!(!out.is_empty());
            assert!(out.contains("dwCSGOInput"));
        }
    }

    #[test]
    fn test_generate_output_files() {
        let patterns = pattern_map();
        let timestamp = "2026-10-06 06:58:22.576223100 UTC";

        for file_type in ["cs", "hpp", "json", "rs", "zig"] {
            let mut out = String::new();
            let mut fmt = Formatter::new(&mut out, 4);

            if file_type != "json" {
                writeln!(&mut fmt, "// Generated using https://github.com/a2x/cs2-dumper").unwrap();
                writeln!(&mut fmt, "// {}\n", timestamp).unwrap();
            }

            match file_type {
                "cs" => patterns.write_cs(&mut fmt).unwrap(),
                "hpp" => patterns.write_hpp(&mut fmt).unwrap(),
                "json" => patterns.write_json(&mut fmt).unwrap(),
                "rs" => patterns.write_rs(&mut fmt).unwrap(),
                "zig" => patterns.write_zig(&mut fmt).unwrap(),
                _ => unreachable!(),
            }

            let file_path = format!("output/patterns.{}", file_type);
            std::fs::write(file_path, out).unwrap();
        }
    }
}
