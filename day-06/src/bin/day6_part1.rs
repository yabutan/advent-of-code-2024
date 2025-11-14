use day_06::{Direction, Guard, InputData, Tracker};

fn main() -> anyhow::Result<()> {
    let text = include_str!("../../data/input.txt");
    //let text = include_str!("../../data/sample.txt");
    let input = InputData::parse(text);
    println!("{:?}", input);

    let mut guard = Guard {
        pos: input.start_pos,
        facing: Direction::Up,
    };
    let mut tracker = Tracker::new(&guard);
    println!("guard: {:?}", guard);

    loop {
        let (is_end, next) = guard.move_next(&input);
        tracker.track(&guard, next.pos);
        guard = next;
        println!("guard: {:?}", guard);

        if is_end {
            break;
        }
    }

    //println!("{:?}", tracker);
    println!("max: {}", tracker.len());
    // max: 5516

    Ok(())
}
