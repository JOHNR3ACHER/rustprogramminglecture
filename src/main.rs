
fn main() {
    // Creating a vector
    let mut nums: Vec<i32> = Vec::new();
    nums.push(5);
    nums.push(6);
    nums.push(7);
    nums.push(8);

    // Accessing vector elements
    if let Some(last) = nums.get(3) {
        println!("Last element is {:?}", last);
    }
    
    // Iterating over vector values
    for num in &nums {
        println!("Num: {}", num);
    }

    // // Using enums to store different types
    // #[derive(Debug)]
    // enum MyData {
    //     Int(i32),
    //     Text(String),
    //     Vector(Vec<i32>),
    // }

    // let mut mix: Vec<MyData> = Vec::new();
    // mix.push(MyData::Int(100));
    // mix.push(MyData::Text("Hello World".to_string()));
    // mix.push(MyData::Vector(vec![1, 2, 5]));

    // for elem in &mix {
    //     println!("Element: {:?}", elem);
    // }
}