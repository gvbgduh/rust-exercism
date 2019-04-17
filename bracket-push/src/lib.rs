pub fn brackets_are_balanced(string: &str) -> bool {
    let mut buf = vec![];

    for item in string.chars() {
        println!("{:?}", item);
        match item {
            '[' | '{' | '(' => {&buf.push(item);},
            ']' if buf.last().unwrap_or(&'?') == &'[' => {&buf.pop();},
            '}' if buf.last().unwrap_or(&'?') == &'{' => {&buf.pop();},
            ')' if buf.last().unwrap_or(&'?') == &'(' => {&buf.pop();},
            ']' | '}' | ')' if buf.last().unwrap_or(&'?') != &item => {&buf.push(item); ;break;},
            ']' | '}' | ')' => {&buf.push(item); break;},
            _ => (),
        };
    }

    buf.is_empty()
}
