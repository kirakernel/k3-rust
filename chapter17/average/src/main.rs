pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            average: 0f64,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }

            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut averaged_collection = AveragedCollection::new();

    for i in 1..=100 {
        averaged_collection.add(i);
    }

    println!("Average: {}", averaged_collection.average());

    for _ in 1..=10 {
        averaged_collection.remove();
    }

    println!("Average: {}", averaged_collection.average());
}
