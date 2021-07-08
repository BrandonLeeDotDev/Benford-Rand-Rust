use std::{
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {

    let mut vec = vec![10;0];

    let mut sum = 0.0;

    loop {
        vec[9] = vec_update(vec[9]);

        let count = vec[9];

        fn time() -> u128 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Before time!")
                .as_nanos()
        }

        let mut start_time = time();

        let pow = ((time() % 89) + 10) as f64 * 0.1;

        sum += pow;

        let mean = sum / count as f64;

        let mut reversed = 0;

        while start_time != 0 {
            reversed = reversed * 10 + start_time % 10;
            start_time /= 10;
        }

        //let num = ((reversed as f64 * (mean * 0.000000000000000149)) + 1.0).powf(pow).trunc() as u128; //0.203

        let num = ((reversed as f64).powf(pow) + reversed as f64) * 0.000000000000001;

        let num_length = num.to_string().chars().count() as u8;

        let first_num = num.to_string()[0..1].parse::<u8>().unwrap();

        fn vec_update(mut vector: u32) -> u32{
            vector += 1;
            vector
        }

        vec[usize::from(first_num - 1)] = vec_update(vec[usize::from(first_num - 1)]);

        fn number(number: u32, count: u32) -> f32 {
            (number as f32 / count as f32) * 100.0
        }

        let space = if num_length >= 1 && num_length <= 9 {"0"} else {""};

        println!("1x {:.2}%, 2x {:.2}%. 3x {:.2}%, 4x {:.2}%, 5x {:.2}%, 6x {:.2}%, 7x {:.2}%, 8x {:.2}%, 9x {:.2}% - Iterations:{} - Digits:{}{} -- Rand:{} ", number(vec[0], count), number(vec[1], count), number(vec[2], count), number(vec[3], count), number(vec[4], count), number(vec[5], count), number(vec[6], count), number(vec[7], count), number(vec[8], count),count, space,num_length, num);

        //println!("Iterations:{} - Digits:{}{} - Rand:{} - Start:{}",count, space,num_length, num, reversed);

        if num == 340282366920938463463374607431768211455.0 {
            println!("Found : {}", num);
            break;
        }

    }
}
