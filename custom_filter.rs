struct FilterCondition {
    my_val: i32,
}

//checks whether the item is divisible by the number
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item % self.my_val == 0
    }
}

fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_items = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            filtered_items.push(*item);
        }
    }

    filtered_items
}

fn main() {
    let collection = vec![58, 12, 78, 91, 46, 142, 30, 27, 109, 33, 7, 59, 116, 83, 130];
    //check which numbers are divisible by 3 in the collection
    let condition = FilterCondition { my_val: 3 };
    println!("the results are: {:?}", custom_filter(&collection, &condition));
}
