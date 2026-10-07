#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Result;

use clap::{ArgAction, Parser};

use log::{LevelFilter, info, warn};

use memflow::prelude::v1::*;

use simplelog::*;

use output::Output;

mod analysis;
mod memory;
mod output;
mod source2;

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    /// The name of the memflow connector to use.
    #[arg(short, long)]
    connector: Option<String>,

    /// Additional arguments to pass to the memflow connector.
    #[arg(short = 'a', long)]
    connector_args: Option<String>,

    /// Path to the CS2 game directory or directory containing DLL files.
    /// If specified or if the game process is not running, scans the DLL files directly from disk.
    #[arg(short = 'd', long)]
    game_dir: Option<PathBuf>,

    /// Force offline mode (scan DLL files directly without attaching to memory).
    #[arg(long)]
    offline: bool,

    /// The types of files to generate.
    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_values = ["cs", "hpp", "json", "rs", "zig"]
    )]
    file_types: Vec<String>,

    /// The number of spaces to use per indentation level.
    #[arg(short, long, default_value_t = 4)]
    indent_size: usize,

    /// The output directory to write the generated files to.
    #[arg(short, long, default_value = "output")]
    output: PathBuf,

    /// The name of the game process.
    #[arg(short, long, default_value = "cs2.exe")]
    process_name: String,

    /// Increase logging verbosity. Can be specified multiple times.
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// Prevent creation of the cs2-dumper.log file.
    #[arg(short, long)]
    no_log_file: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let level_filter = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    // Create the log file by default.
    if !args.no_log_file {
        loggers.push(WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("cs2-dumper.log")?,
        ));
    }

    CombinedLogger::init(loggers)?;

    let now = Instant::now();

    if args.offline || args.game_dir.is_some() {
        info!("running in offline DLL scanning mode...");
        let (result, build_number) = analysis::analyze_offline(args.game_dir.as_deref())?;
        let output = Output::new(&args.file_types, args.indent_size, &args.output, &result)?;
        output.dump_offline(build_number)?;
        info!("offline analysis completed in {:.2?}", now.elapsed());
        return Ok(());
    }

    let conn_args = args
        .connector_args
        .map(|s| ConnectorArgs::from_str(&s).expect("unable to parse connector arguments"))
        .unwrap_or_default();

    let os_res = match args.connector {
        Some(conn) => {
            let mut inventory = Inventory::scan();

            inventory
                .builder()
                .connector(&conn)
                .args(conn_args)
                .os("win32")
                .build()
                .map_err(|e| anyhow::anyhow!("{}", e))
        }
        None => {
            #[cfg(windows)]
            {
                memflow_native::create_os(&OsArgs::default(), LibArc::default())
                    .map_err(|e| anyhow::anyhow!("{}", e))
            }
            #[cfg(not(windows))]
            {
                anyhow::bail!("no connector specified")
            }
        }
    };

    let mut os = match os_res {
        Ok(os) => os,
        Err(err) => {
            warn!(
                "could not initialize memflow OS connector: {}. Falling back to offline DLL scanning...",
                err
            );

            let (result, build_number) = analysis::analyze_offline(args.game_dir.as_deref())?;
            let output = Output::new(&args.file_types, args.indent_size, &args.output, &result)?;
            output.dump_offline(build_number)?;
            info!("offline analysis completed in {:.2?}", now.elapsed());
            return Ok(());
        }
    };

    match os.process_by_name(&args.process_name) {
        Ok(mut process) => {
            let result = analysis::analyze_all(&mut process)?;
            let output = Output::new(&args.file_types, args.indent_size, &args.output, &result)?;

            output.dump_all(&mut process)?;

            info!("analysis completed in {:.2?}", now.elapsed());
        }
        Err(err) => {
            warn!(
                "could not attach to game process ({}): {}. Falling back to offline DLL scanning...",
                args.process_name, err
            );

            let (result, build_number) = analysis::analyze_offline(args.game_dir.as_deref())?;
            let output = Output::new(&args.file_types, args.indent_size, &args.output, &result)?;

            output.dump_offline(build_number)?;

            info!("offline analysis completed in {:.2?}", now.elapsed());
        }
    }

    Ok(())
}
