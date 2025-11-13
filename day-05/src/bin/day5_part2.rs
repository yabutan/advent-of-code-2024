use day_05::InputData;

fn main() -> anyhow::Result<()> {
    let text = include_str!("../../data/input.txt");
    //let text = include_str!("../../data/sample.txt");
    let input = InputData::parse(text);
    let point = cal_point(&input);
    println!("point: {}", point);
    // point: 3062

    Ok(())
}

fn cal_point(input: &InputData) -> i32 {
    let mut count = 0;
    for record in &input.records {
        if !record.is_correct(input) {
            println!("{:?}", record);
            let mut record = record.clone();
            record.sort(input);

            println!(" >>> {:?}", record);
            count += record.get_center();
        }
    }
    count
}
