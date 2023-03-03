use perfect::constants;



fn main() {
    let mut numbers: Vec<Vec<i32>> = (0..constants::NUM_NUMBERS).map(|_| vec![]).collect();
    constants::PERFECT_NUMBERS.iter().for_each(|i| numbers[i].push(*i));

    let mut available: Vec<i32> = vec!();
    constants::PERFECT_NUMBERS.iter().for_each(|i| available.push(*i));

    while !available.is_empty() {
        let current = available.pop().unwrap();

    }

    println!("{:?}", numbers);
}
