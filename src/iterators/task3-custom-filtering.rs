fn main() {
    let collection = vec![1,2,3,4,5];
    let filter_condition = FilterCondition { condition: 3 };
    let filtered = custom_filter(&collection, &filter_condition);
    println!("Filtered collection: {:?}", filtered);
}

struct FilterCondition {
    condition: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        &self.condition == item
    }
}

fn custom_filter (collection: &Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    collection.iter().cloned().filter(|item|filter.is_match(item)).collect()
}