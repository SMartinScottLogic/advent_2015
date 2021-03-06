use d1p1::load;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let solution = load("input/day1.input")?;
    println!("{}", solution.answer());
    Ok(())
}
