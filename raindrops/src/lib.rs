pub fn raindrops(n: u32) -> String {
    let a = [3, 5, 7];
    let mut s = String::new();

    for el in a.iter() {
        if n % el == 0 {
            match el {
                3 => {s = s + "Pling";},
                5 => {s = s + "Plang";},
                7 => {s = s + "Plong";},
                _ => (),
            }
        }
    }

    if s.is_empty() {
        n.to_string()
    } else {
        s
    }
}

//pub fn raindrops(n: u32) -> String {
//    let a = [3, 5, 7];
//    let mut s = String::new();
//
//    for el in a.iter() {
//        if n % el == 0 {
//            match el {
//                3 => {s = s + "Pling";},
//                5 => {s = s + "Plang";},
//                7 => {s = s + "Plong";},
//                _ => (),
//            }
//        }
//    }
//
//    if s.is_empty() {
//        n.to_string()
//    } else {
//        s
//    }
//}