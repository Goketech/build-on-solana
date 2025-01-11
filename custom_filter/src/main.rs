fn main() {
    println!("Welcome to my Filter program!");

    let collection = vec!["python", "javascript", "rust", "java", "c"];
    let filter_condition = FilterCondition { filter: "java".to_string() };

    let result = custom_filter(collection, &filter_condition);
    println!("Result: {:?}", result);
}

struct FilterCondition {
    filter: String,
}

impl FilterCondition {
    fn is_match(&self, item: &str) -> bool {
        item.contains(&self.filter)
    }
}

fn custom_filter<'a>(collection: Vec<&'a str>, filter_condition: &'a FilterCondition ) -> Vec<&'a str> {
    let mut result = Vec::new();
    for item in collection {
        if filter_condition.is_match(item) {
            result.push(item);
        }
    }
    result
}