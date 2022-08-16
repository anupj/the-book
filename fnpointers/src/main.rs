fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug, PartialEq)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("the answer is : {}", answer);

    // turn a vector of numbers into a vector of strings
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // alternatively use the ToString::to_string function
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    assert_eq!(list_of_strings, list_of_strings2);

    let list_of_statuses: Vec<Status> = (0u32..6).map(Status::Value).collect();
    let statuses = vec![Status::Value(0),
                        Status::Value(1),
                        Status::Value(2),
                        Status::Value(3),
                        Status::Value(4),
                        Status::Value(5)];
    assert_eq!(list_of_statuses.get(0), statuses.get(0));
    assert_eq!(list_of_statuses, statuses);
                        
}
