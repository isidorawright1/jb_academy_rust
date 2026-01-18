//Here is the infinite loop that loops through 1000 numbers and when it gets to number 555
// it prints 'the number 555 has been found' and exits the loop
//I am not sure if these numbers are supposed to be randomly generated or if I iterate
//  through a count...
//set the initial count to 1 since it is the max amount of numbers to run through
use rand::Rng; //added rand import to toml file
fn main() {
    //random number version

    //iteration version
    /*let mut count: i32 = 1;
    const MAX : i32 = 1000;
    loop {

        if count == 555 {
            println!("Number 555 found");
            break;
        }

        if count == MAX {
            println!("Number not found");
            break
        }

        count += 1;
    }*/
}


/*fn main() {
    let number_of_vowels = length_of_string("Isidora".to_string());
    println!("The number of vowels in the string is {}", number_of_vowels);
}

fn length_of_string(string_value :String) -> i32
{
    let mut count = 0;
    for i in string_value.chars() {
        match i {
            'a' | 'A' => count += 1,
            'e' | 'E' => count += 1,
            'i' | 'I' => count += 1,
            'o' | 'O' => count += 1,
            'u' | 'U' => count += 1,
            _ => count += 0,
        }
    }
    count
}*/

//Old code to be refactored. DO NOT UNCOMMENT BELOW CODE
/*fn length_of_string(string_value :String) -> i32
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
}*/
