use d1p2::load;
use anyhow::Result;
use log::{error, info};
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("day1.input")?;
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