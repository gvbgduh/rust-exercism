const REPLIES: [&'static str; 5] = [
    "Sure.",
    "Whoa, chill out!",
    "Calm down, I know what I'm doing!",
    "Fine. Be that way!",
    "Whatever.",
];

pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    let is_question = msg.ends_with("?");
    let is_text = msg.chars().any(|c| c.is_alphabetic());
    let is_yelling = msg.to_uppercase() == msg && is_text;

    let is_empty = msg.is_empty();
    match (is_text, is_yelling, is_question, is_empty) {
        (_    , false, true , false) => &REPLIES[0],
        (true , true , false, false) => &REPLIES[1],
        (true , true , true , false) => &REPLIES[2],
        (false, false, false, false) => &REPLIES[4],
        (false, _    , _    , _    ) => &REPLIES[3],
        _                            => &REPLIES[4],
    }
}


//ub fn reply(message: &str) -> &str {
//    // Check for yelling
//    let is_yelling = message.contains(char::is_alphabetic) && message == message.to_uppercase();
//    // Pattern match mesage
//    match message.trim() {
//        // Empty message
//        m if m.is_empty() => "Fine. Be that way!",
//        // Question and yelling
//        m if m.ends_with("?") && is_yelling => "Calm down, I know what I'm doing!",
//        // Question
//        m if m.ends_with("?") => "Sure.",
//        // Just yelling
//        m if is_yelling => "Whoa, chill out!",
//        // Anything else
//        _ => "Whatever."
//    }
//}


//pub fn reply(message: &str) -> &str {
//  let msg = message.trim();
//  if msg.len() == 0 {
//    return "Fine. Be that way!";
//  }
//
//  let has_letters: bool = msg.chars().any(|x| x.is_alphabetic());
//  let is_shouting: bool = has_letters && msg.to_uppercase() == msg;
//  let is_question: bool = msg.chars().last() == Some('?');
//
//  match (is_shouting, is_question) {
//    (true, true) => "Calm down, I know what I'm doing!",
//    (true, _) => "Whoa, chill out!",
//    (_, true) => "Sure.",
//    (_, _) => "Whatever.",
//  }
//}


//fn is_question(message: &str) -> bool {
//    match message.trim().chars().last() {
//        Some('?') => true,
//        _         => false
//    }
//}
//
//fn is_shout(message: &str) -> bool {
//    let mut alpha = message.trim().chars().filter(|c| c.is_alphabetic()).peekable();
//    alpha.peek().is_some() && alpha.all(|c| c.is_uppercase())
//}
//
//fn is_silence(message: &str) -> bool {
//    message.trim().is_empty()
//}
//
//pub fn reply(message: &str) -> &str {
//    match (is_question(message), is_shout(message), is_silence(message)) {
//        ( true,  true,    _) => "Calm down, I know what I'm doing!",
//        ( true, false,    _) => "Sure.",
//        (false,  true,    _) => "Whoa, chill out!",
//        (    _,     _, true) => "Fine. Be that way!",
//        (    _,     _,    _) => "Whatever."
//    }
//}