use std::time::{SystemTime, UNIX_EPOCH};
extern crate num;
use num::bigint::BigUint;
/// This (P)RNG confirms to Benford's law suggesting its not just pseudo random.
fn main() {
    let mut vec = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    fn rev_num(mut number: u128) -> u128 {
        let mut reversed = 0;

        while number != 0 {
            reversed = reversed * 10 + number % 10;
            number /= 10;
        }
        reversed
    }

    fn return_rand(number: u128) -> BigUint {
        let pow = (time() % 211) + 1;
        BigUint::from(rev_num(number)).pow(pow as u32) * BigUint::from(rev_num(time()))
        //(BigUint::from(rev_num(time())) + (BigUint::from(rev_num(number)) / BigUint::from(pow)) // roughly even
        //     .pow(pow as u32 % 89)
        //     * BigUint::from(rev_num(time()))
    }

    fn time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Before time!")
            .as_nanos()
    }

    fn vec_update(mut vector: u32) -> u32 {
        vector += 1;
        vector
    }

    fn number(number: u32, count: u32) -> f32 {
        (number as f32 / count as f32) * 100.0
    }

    loop {
        vec[9] = vec_update(vec[9]);

        let count = vec[9];

        let num = return_rand(time());

        let num_length = num.to_string().chars().count() as u64;

        let first_num = num.to_string()[0..1].parse::<u8>().unwrap();

        vec[usize::from(first_num - 1)] = vec_update(vec[usize::from(first_num - 1)]);

        println!("1x {:.2}%, 2x {:.2}%. 3x {:.2}%, 4x {:.2}%, 5x {:.2}%, 6x {:.2}%, 7x {:.2}%, 8x {:.2}%, 9x {:.2}% - Iterations:{} - Digits:{}", number(vec[0], count), number(vec[1], count), number(vec[2], count), number(vec[3], count), number(vec[4], count), number(vec[5], count), number(vec[6], count), number(vec[7], count), number(vec[8], count),count,num_length);
    }
}
