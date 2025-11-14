use day_06::{Direction, Guard, InputData, Tracker};

fn main() -> anyhow::Result<()> {
    let text = include_str!("../../data/input.txt");
    //let text = include_str!("../../data/sample.txt");
    let input = InputData::parse(text);
    println!("{:?}", input);

    // 元々の経路を抽出
    let tracker = get_tracker(&input)?;
    println!("max {}", tracker.len());

    // 経路に壁を追加してみて、ループが発生するかどうかを調べる
    let mut stuck_count = 0;
    for &obstruction in tracker.positions() {
        if input.start_pos == obstruction {
            // スタート地点は候補にしない
            continue;
        }

        let input = input.clone().add_obstruction(obstruction);
        if simulate_loop(&input) {
            println!("simulate additional wall {:?}", obstruction);
            stuck_count += 1;
        }
    }
    println!("count: {}", stuck_count);
    // count: 2008

    Ok(())
}

fn simulate_loop(input: &InputData) -> bool {
    let mut guard = Guard {
        pos: input.start_pos,
        facing: Direction::Up,
    };
    let mut tracker = Tracker::new(&guard);

    loop {
        let (is_end, next) = guard.move_next(input);
        if is_end {
            return false;
        }

        if guard.pos == next.pos {
            // 同じ座標で向きだけ変わってる場合は、トラッキングせずに次を進める
            guard = next;
            continue;
        }

        if tracker.track(&guard, next.pos) {
            return true;
        }

        guard = next;
    }
}

fn get_tracker(input: &InputData) -> anyhow::Result<Tracker> {
    let mut guard = Guard {
        pos: input.start_pos,
        facing: Direction::Up,
    };
    let mut tracker = Tracker::new(&guard);

    loop {
        let (is_end, next) = guard.move_next(input);
        tracker.track(&guard, next.pos);
        guard = next;
        if is_end {
            break;
        }
    }
    Ok(tracker)
}
