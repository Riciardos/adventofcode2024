use std::thread;
use crate::day1::{day1, day1_star2};
use crate::day10::day10;
use crate::day2::{day2, day2_star2};
use crate::day3::{day3, day3_part2};
use crate::day4::{day4, day4_star2};
use crate::day5::{day5, day5_star2};
use crate::day6::{day6, day6_part2};
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::{day9, day9_part2};

mod day1;
mod day2;
mod util;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    parallel();
    sequential();
}

fn sequential() {
    use std::time::Instant;
    let now = Instant::now();

    day1();
    day1_star2();
    day2();
    day2_star2();
    day3();
    day3_part2();
    day4();
    day4_star2();
    day5();
    day5_star2();
    day6();
    day6_part2();
    day7(); // function changed in place to do part 2
    day8();
    day9();
    day9_part2();
    day10();

    let elapsed = now.elapsed();
    println!("Elapsed sequential: {:.2?}", elapsed);
}

fn parallel() {
    use std::time::Instant;
    let now = Instant::now();

    let join_1 = thread::spawn(|| day1::day1());
    let join_2 = thread::spawn(|| day1_star2());
    let join_3 = thread::spawn(|| day2::day2());
    let join_4 = thread::spawn(|| day2_star2());
    let join_5 = thread::spawn(|| day3::day3());
    let join_6 = thread::spawn(|| day4());
    let join_7 = thread::spawn(|| day4_star2());
    let join_8 = thread::spawn(|| day5());
    let join_9 = thread::spawn(|| day5_star2());
    let join_10 = thread::spawn(|| day6());
    let join_11 = thread::spawn(|| day6_part2());
    let join_12 = thread::spawn(|| day7());
    let join_13 = thread::spawn(|| day8());
    let join_14 = thread::spawn(|| day9());
    let join_15 = thread::spawn(|| day9_part2());


    join_1.join().expect("thread 1 panicked");
    join_2.join().expect("thread 2 panicked");
    join_3.join().expect("thread 3 panicked");
    join_4.join().expect("thread 4 panicked");
    join_5.join().expect("thread 5 panicked");
    join_6.join().expect("thread 6 panicked");
    join_7.join().expect("thread 7 panicked");
    join_8.join().expect("thread 8 panicked");
    join_9.join().expect("thread 9 panicked");
    join_10.join().expect("thread 10 panicked");
    join_11.join().expect("thread 11 panicked");
    join_12.join().expect("thread 12 panicked");
    join_13.join().expect("thread 13 panicked");
    join_14.join().expect("thread 14 panicked");
    join_15.join().expect("thread 15 panicked");

    let elapsed = now.elapsed();
    println!("Elapsed parrallel: {:.2?}", elapsed);
}
