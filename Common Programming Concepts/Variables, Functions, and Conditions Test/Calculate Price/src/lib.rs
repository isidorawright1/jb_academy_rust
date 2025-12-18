pub fn calculate_price(num_apples :i32) -> i32 {
    if num_apples >= 40 {
        num_apples
    }
    else {
        num_apples * 2
    }
}

fn main() {
    let price = calculate_price(5);
    println!("The price of apples is: {}", price);
}