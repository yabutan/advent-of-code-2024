use glam::IVec2;
use std::collections::{HashMap, HashSet};
use std::intrinsics::copy_nonoverlapping;

#[derive(Debug)]
struct InputData {
    order_map: HashMap<i32, HashSet<i32>>,
    records: Vec<Vec<i32>>,
}

impl InputData {
    fn parse(text: &str) -> Self {
        let (top, bottom) = text.split_once("\n\n").unwrap();
        let mut order_map = HashMap::new();
        top.lines().for_each(|line| {
            let (left, right) = line.split_once("|").unwrap();
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();
            order_map
                .entry(left)
                .or_insert(HashSet::new())
                .insert(right);
        });

        let mut records = Vec::new();

        bottom.lines().for_each(|line| {
            let mut record = Vec::new();
            line.split(",").for_each(|s| {
                record.push(s.parse::<i32>().unwrap());
            });
            records.push(record);
        });

        Self { order_map, records }
    }
}

fn main() -> anyhow::Result<()> {
    let text = include_str!("../../data/input.txt");
    //let text = include_str!("../../data/sample.txt");
    let input = InputData::parse(text);
    let point = cal_point(&input);
    println!("point: {}", point);
    // point: 5948

    Ok(())
}

fn cal_point(input: &InputData) -> i32 {
    let mut count = 0;
    for record in &input.records {
        if is_correct(input, record) {
            count += record[record.len() / 2];
        }
    }
    count
}

fn is_correct(input: &InputData, record: &[i32]) -> bool {
    let index_max: HashMap<i32, usize> = record.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    let mut correct = true;
    for (i, &x) in record.iter().enumerate() {
        let Some(orders) = input.order_map.get(&x) else {
            continue;
        };

        for &o in orders {
            let Some(&j) = index_max.get(&o) else {
                continue;
            };

            if i > j {
                correct = false;
                break;
            }
        }
    }
    correct
}
