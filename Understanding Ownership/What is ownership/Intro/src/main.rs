fn main() {
    let number_of_vowels = length_of_string("Hello".to_string());
    println!("The number of vowels in the string is {}", number_of_vowels);
}

fn length_of_string(string_value :String) -> i32
{
    let mut count = 0;
    let array_of_vowels = ['a', 'e', 'i', 'o', 'u'];
    for i in string_value.chars(){
        if (i in array_of_vowels)
        {
            count += 1;
        }
    }
    count
}
