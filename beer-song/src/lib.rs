const ZERO: &'static str = "No more bottles of beer on the wall, \
    no more bottles of beer.\nGo to the store and buy some more, \
    99 bottles of beer on the wall.\n";
const ONE: &'static str = "1 bottle of beer on the wall, 1 bottle \
    of beer.\nTake it down and pass it around, no more bottles of \
    beer on the wall.\n";
const TWO: &'static str = "2 bottles of beer on the wall, 2 bottles \
    of beer.\nTake one down and pass it around, 1 bottle of beer on \
    the wall.\n";


pub fn verse(n: i32) -> String {
    match Some(n) {
        Some(0)   => String::from(ZERO),
        Some(1)   => String::from(ONE),
        Some(2)   => String::from(TWO),
        Some(num) => format!(
            "{one} bottles of beer on the wall, {one} bottles of beer.\n\
             Take one down and pass it around, {two} bottles of beer on the wall.\n",
            one=num,
            two=num - 1
        ),
        None      => String::new(),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();

    for n in (end..=start).rev() {
        let line = verse(n);
        song.push_str(&line);
        if n > end {
            song.push_str("\n");
        }
    }

    song
}


//pub fn verse(n: u32) -> String {
//    match n {
//        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
//        1 => format!("{0} on the wall, {0}.\nTake it down and pass it around, no more bottles of beer on the wall.\n", beer(n)),
//        2...99 => format!("{0} on the wall, {0}.\nTake one down and pass it around, {1} on the wall.\n", beer(n), beer(n - 1)),
//        _ => panic!("{} is not a valid verse", n)
//    }
//}
//
//fn beer(n: u32) -> String {
//    match n {
//        1 => "1 bottle of beer".to_string(),
//        2...99 => format!("{} bottles of beer", n),
//        _ => panic!("{} is not a valid number of beers")
//    }
//}
//
//pub fn sing(start_verse: u32, end_verse: u32) -> String {
//    let range = (end_verse..start_verse + 1).rev();
//    range.map(|n| verse(n)).collect::<Vec<_>>().join("\n")
//}


//extern crate itertools;
//
//use itertools::Itertools;
//
//
//pub fn verse(verse: i32) -> String {
//    match verse {
//        0 => return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
//
//        1 => return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
//
//        2 => return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
//
//        _ => return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", verse, verse, verse-1).to_string(),
//    }
//}
//
//pub fn sing(from: i32, to: i32) -> String {
//    (to..from+1).rev()
//                .into_iter()
//                .map(verse)
//                .join("\n")
//}


//const SPECIAL_VERSES: [&'static str; 3] = [
//  "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
//  "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
//  "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n",
//];
//
//pub fn verse(number: usize) -> String {
//  if number < SPECIAL_VERSES.len() {
//    SPECIAL_VERSES[number].to_string()
//  } else {
//    format!("{current} bottles of beer on the wall, {current} bottles of beer.\nTake one down and pass it around, {next} bottles of beer on the wall.\n", current=number, next=number - 1)
//  }
//}
//
//pub fn sing(first: usize, last: usize) -> String {
//  let verses: Vec<_> = (last..first + 1).rev().map(verse).collect();
//  verses.join("\n")
//}
//
//My guess is the type inference for collect() posed a problem. (last..first + 1) .rev() .map(verse) .collect::<vec>&gt;() .join("\n") </vec> `


//#![feature(inclusive_range_syntax)]
//
//pub fn verse(n: i32) -> String {
//    let bottle_fmt = |n| {
//        match n {
//            0 => format!("no more bottles"),
//            1 => format!("1 bottle"),
//            _ => format!("{} bottles", n),
//        }
//    };
//
//    match n {
//        1 ... 99 => format!("{n_bottles} of beer on the wall, {n_bottles} of beer.\nTake {one_or_it} down and pass it around, {n_less_1_bottles} of beer on the wall.\n",
//                            n_bottles = bottle_fmt(n),
//                            one_or_it = if n == 1 { "it" } else { "one" },
//                            n_less_1_bottles = bottle_fmt(n - 1)),
//        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
//        _ => unimplemented!(),
//    }
//}
//
//pub fn sing(start: i32, end: i32) -> String {
//    (end..=start).rev()
//                 .map(&verse)
//                 .collect::<Vec<_>>()
//                 .join("\n")
//}