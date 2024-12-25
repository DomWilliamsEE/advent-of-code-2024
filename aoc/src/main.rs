use anyhow::{bail, Context, Result};
use common::itertools::Itertools;
use common::{CaseEntrypointFn, PartNumber};
use env_logger::Env;
use libloading::{Library, Symbol};
use log::{error, info, warn};
use std::env::args;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

#[derive(Debug, Clone)]
pub struct Args {
    /// 0 for all
    pub day: u32,
    pub year: u32,
    /// All parts if not set
    pub part: Option<PartNumber>,
    pub only_solutions: bool,
    pub case: Option<u32>,
}

fn do_main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut args = Args::parse()?;

    info!("lets go: {args:?}");

    let days = if args.day == 0 {
        (1..=25).collect_vec()
    } else {
        vec![args.day]
    };

    for day in days {
        args.day = day;
        let input = format!("inputs/{}-{:02}", args.year, args.day);
        info!("reading input from {input}");

        let do_it = || {
            let input = std::fs::read_to_string(&input)
                .with_context(|| format!("Failed to read input from {input}"))?;

            ensure_solution_built(&args).context("Failed to ensure solution is built")?;

            run_solution(args.clone(), input.trim_end())
        };

        if let Err(e) = do_it() {
            error!("failed to run for day {day}: {e:#}");
        }
    }
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
            let func: Symbol<CaseEntrypointFn> = lib
                .get(b"run_cases_entrypoint")
                .context("Failed to load run_cases_entrypoint symbol")?;

            let part_filter = args.part.map(|p| p as u8).unwrap_or(0);
            let case_filter = args.case.unwrap_or(0);

            info!("calling run_cases entrypoint");
            let res = func(
                input.as_ptr(),
                input.len(),
                part_filter,
                case_filter,
                args.only_solutions,
            );
            if res {
                info!("all cases passed");
            } else {
                warn!("some cases failed");
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
        let mut day = 0;
        let mut year = None;
        let mut part = None;
        let mut only_solutions = false;
        let mut case = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--day" => {
                    day = args
                        .next()
                        .context("--day requires a number")?
                        .parse()
                        .context("day must be a valid number")?;
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
                "--case" => {
                    case = Some(
                        args.next()
                            .context("--case requires a number")?
                            .parse()
                            .context("case must be a valid number")?,
                    );
                }
                "--only-solutions" => only_solutions = true,
                _ => bail!("unknown argument"),
            }
        }

        Ok(Args {
            day,
            year: year.context("--year is required")?,
            part,
            only_solutions,
            case,
        })
    }
}
