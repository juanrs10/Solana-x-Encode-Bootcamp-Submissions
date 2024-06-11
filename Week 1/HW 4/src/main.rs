use std::collections::HashMap;

fn main() {
    println!("Welcome to my program that does stuff for free guys");
    println!("{:?}", two_sum(vec![2, 3, 4, 5], 9));
    fizz_buzz();
}

fn fizz_buzz() {
    let mut count = 0;
    for i in 1..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            count += 1;
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        }
    }
    println!("The number of times 'fizz buzz' occurred: {}", count);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut numbers_map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = numbers_map.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        numbers_map.insert(num, i);
    }

    vec![] // Exit.
}
