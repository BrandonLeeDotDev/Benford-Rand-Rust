use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    thread,
};

fn main() {
    let mut count = 1;

    let mut vec = vec![0,0,0,0,0,0,0,0,0];

    loop {
        // --------- v Necessary for Algo v --------- //
        fn time(switch: bool) -> u128 {
            if switch == true {
                SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Before time!")
                .as_nanos()
            } else {
                SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Before time!")
                .as_millis()
            }
        }

        let start_time = time(true);

        thread::sleep(Duration::from_nanos(1));

        let mut loop_time = (time(true) % 89) + 10;

        let pow = loop_time as f64 * 0.1;

        while loop_time > 0 {
            loop_time *= loop_time;
            thread::sleep(Duration::from_nanos(1));
            loop_time /= loop_time;
            loop_time -= 1;
        }

        let end_time = time(false);

        let mut reversed = 0;

        let mut calc_time = start_time - end_time;

        while calc_time != 0 {
            reversed = reversed * 10 + calc_time % 10;
            calc_time /= 10;
        }

        let num = ((reversed as f64 * 0.0000000000000001) + 1.0).powf(pow).trunc() as u128;
        
        // --------- ^ Necessary for Algo ^ --------- //
        
        // -------------- V For Stats v ------------- //

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

        println!("1x {:.2}%, 2x {:.2}%. 3x {:.2}%, 4x {:.2}%, 5x {:.2}%, 6x {:.2}%, 7x {:.2}%, 8x {:.2}%, 9x {:.2}% -- Iterations:{} - Digits:{}{} -- Rand:{}", number(vec[0], count), number(vec[1], count), number(vec[2], count), number(vec[3], count), number(vec[4], count), number(vec[5], count), number(vec[6], count), number(vec[7], count), number(vec[8], count),count,space,num_length, num);

        count += 1;
        
        // -------------- ^ For Stats ^ ------------- //
        
        // -------------- v Max Limit v ------------- //
        
        if num == 340282366920938463463374607431768211455 {
            println!("Error Max : {}", num);
            break;
        }

    }
}
