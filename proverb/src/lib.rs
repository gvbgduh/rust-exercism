use std::iter;

pub fn build_proverb(list: &[&str]) -> String {

    if list.is_empty() {
        return String::new()
    }

    list.windows(2)  // or .zip(list.iter().skip(1))
        .map(|x| line_fmt(x[0], x[1]))  // |x, y| for zip
        .chain(iter::once(
            format!("And all for the want of a {}.", list[0])
        ))
        .collect::<Vec<_>>()
        .join("\n")
}

fn line_fmt(first: &str, second: &str) -> String {
    format!("For want of a {} the {} was lost.", first, second)
}
