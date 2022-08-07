use anyhow::Result;
use day22::load;
use env_logger::Env;
use log::{error, info};
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut solution = load("input/day22.input")?;
    info!(
        "{}{}: {:?}",
        Paint::masked("🎄 "),
        Paint::bold(Paint::yellow("solution")),
        solution
    );
    solution.analyse();
    match solution.answer_part1() {
        Some(answer) => info!(
            "{}part1 answer is {:?}",
            Paint::masked("🎅 "),
            Paint::bold(Paint::red(answer))
        ),
        _ => error!("{}No answer to part1", Paint::masked("🎅 ")),
    }

    match solution.answer_part2() {
        Some(answer) => info!(
            "{}part2 answer is {:?}",
            Paint::masked("🎅 "),
            Paint::bold(Paint::red(answer))
        ),
        _ => error!("{}No answer to part2", Paint::masked("🎅 ")),
    }

    Ok(())
}
