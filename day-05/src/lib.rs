use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct InputData {
    pub order_map: HashMap<i32, HashSet<i32>>,
    pub records: Vec<Record>,
}

impl InputData {
    pub fn parse(text: &str) -> Self {
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
            records.push(Record(record));
        });

        Self { order_map, records }
    }
}

#[derive(Debug, Clone)]
pub struct Record(pub Vec<i32>);

impl Record {
    pub fn is_correct(&self, input: &InputData) -> bool {
        let index_max: HashMap<i32, usize> =
            self.0.iter().enumerate().map(|(i, &x)| (x, i)).collect();

        let mut correct = true;
        for (i, &x) in self.0.iter().enumerate() {
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

    pub fn get_center(&self) -> i32 {
        let i = self.0.len() / 2;
        self.0[i]
    }

    pub fn sort(&mut self, input: &InputData) {
        self.0.sort_by(|a, b| {
            // a, b の関係性を示す定義を参照する
            if let Some(orders) = input.order_map.get(a)
                && orders.contains(b)
            {
                return Ordering::Less;
            }
            if let Some(orders) = input.order_map.get(b)
                && orders.contains(a)
            {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
    }
}
