use anyhow::{bail, Context, Result};
use common::{ExemplarEntrypointFn, PartNumber, SolutionEntrypointFn};
use env_logger::Env;
use libloading::{Library, Symbol};
use log::{error, info, warn};
use std::env::args;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

#[derive(Debug)]
pub struct Args {
    pub day: u32,
    pub year: u32,
    /// All parts if not set
    pub part: Option<PartNumber>,
    pub only_solutions: bool,
}

fn do_main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse()?;

    info!("lets go: {args:?}");

    let input = format!("inputs/{}-{:02}", args.year, args.day);
    info!("reading input from {input}");

    let input = std::fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input from {input}"))?;

    ensure_solution_built(&args).context("Failed to ensure solution is built")?;

    run_solution(args, &input)?;
    Ok(())
}

fn main() -> ExitCode {
    match do_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{e:#}");
            ExitCode::FAILURE
        }
    }
}

fn solution_format(args: &Args) -> String {
    format!("aoc_{}_{:02}", args.year, args.day,)
}

fn ensure_solution_built(args: &Args) -> Result<()> {
    let project_dir =
        std::env::var("CARGO_MANIFEST_DIR").context("Failed to get CARGO_MANIFEST_DIR")?;

    let profile_arg = match std::env::current_exe()
        .context("Failed to get current exe path")?
        .parent()
        .context("Failed to get parent of current exe path")?
        .file_name()
        .context("Failed to get file name of parent of current exe path")?
        .to_str()
        .context("Failed to convert file name of parent of current exe path to str")?
    {
        "debug" => None,
        "release" => Some("--release"),
        _ => {
            return Err(anyhow::anyhow!(
                "did not find debug or release in current binary dir"
            ))
        }
    };

    info!("running cargo build on solution crate");
    let output = Command::new("cargo")
        .current_dir(project_dir)
        .arg("build")
        .args(["--package", &solution_format(args).replace('_', "-")])
        .args(profile_arg)
        .output()
        .context("Failed to execute cargo build")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Cargo build failed: {}", stderr);
    }

    Ok(())
}

fn run_solution(args: Args, input: &str) -> Result<()> {
    let lib_path = PathBuf::from(
        std::env::current_exe()
            .context("Failed to get current exe path")?
            .parent()
            .context("Failed to get parent of current exe path")?,
    )
    .join(format!("lib{}.so", solution_format(&args)));

    info!("loading solution from {}", lib_path.display());

    unsafe {
        let lib = Library::new(lib_path).context("Failed to load solution library")?;

        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
            if args.only_solutions {
                let func: Symbol<SolutionEntrypointFn> = lib
                    .get(b"solution_entrypoint")
                    .context("Failed to load get_solution symbol")?;

                let parts = args
                    .part
                    .map(|p| [p].to_vec())
                    .unwrap_or(vec![PartNumber::Part1, PartNumber::Part2]);

                for part in parts {
                    info!("calling solution entrypoint with part {part:?}");
                    let res = func(input.as_ptr(), input.len(), part as u8);
                    info!("solution to part {part:?}: {res}");
                }
            } else {
                let func: Symbol<ExemplarEntrypointFn> = lib
                    .get(b"run_exemplars_entrypoint")
                    .context("Failed to load run_exemplars_entrypoint symbol")?;

                let part_filter = args.part.map(|p| p as u8).unwrap_or(0);

                info!("calling run_exemplars entrypoint");
                let res = func(input.as_ptr(), input.len(), part_filter);
                if res {
                    info!("all exemplars passed");
                } else {
                    warn!("some exemplars failed");
                }
            }

            Ok(())
        }))
        .map_err(|e| {
            if let Some(s) = e.downcast_ref::<String>() {
                anyhow::anyhow!("Solution panicked: {}", s)
            } else if let Some(s) = e.downcast_ref::<&str>() {
                anyhow::anyhow!("Solution panicked: {}", s)
            } else {
                anyhow::anyhow!("Solution panicked with unknown error")
            }
        })
        .context("Solution panicked")??;

        Ok(())
    }
}

impl Args {
    pub fn parse() -> Result<Self> {
        let mut args = args().skip(1);
        let mut day = None;
        let mut year = None;
        let mut part = None;
        let mut only_solutions = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--day" => {
                    day = Some(
                        args.next()
                            .context("--day requires a number")?
                            .parse()
                            .context("day must be a valid number")?,
                    );
                }
                "--year" => {
                    year = Some(
                        args.next()
                            .context("--year requires a number")?
                            .parse()
                            .context("year must be a valid number")?,
                    );
                }
                "--1" => {
                    part = Some(PartNumber::Part1);
                }
                "--2" => {
                    part = Some(PartNumber::Part2);
                }
                "--only-solutions" => only_solutions = true,
                _ => bail!("unknown argument"),
            }
        }

        Ok(Args {
            day: day.context("--day is required")?,
            year: year.context("--year is required")?,
            part,
            only_solutions,
        })
    }
}
