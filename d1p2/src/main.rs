use anyhow::Result;
use d1p2::load;
use env_logger::Env;
use log::{error, info};
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut solution = load("input/day1.input")?;
    info!(
        "{}{}: {:?}",
        Paint::masked("🎄 "),
        Paint::bold(Paint::yellow("solution")),
        solution
    );
    solution.analyse();
    match solution.answer() {
        Some(answer) => info!(
            "{}answer is {:?}",
            Paint::masked("🎅 "),
            Paint::bold(Paint::red(answer))
        ),
        _ => error!("{}No answer to the problem", Paint::masked("🎅 ")),
    }

    Ok(())
}
