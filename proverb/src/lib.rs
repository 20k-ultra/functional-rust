pub fn build_proverb(list: &[&str]) -> String {
    let mut iterator = list.iter().peekable();
    let mut proverbs = String::new();
    while let Some(value) = iterator.next() {
        if iterator.peek() == None {
            proverbs.push_str(&line(list[0], None))
        } else {
            proverbs.push_str(&line(value, iterator.peek()));
        }
    }
    proverbs
}

fn line(want_a: &str, was_lost: Option<&&&str>) -> String {
    match (want_a, was_lost) {
        (sub1, Some(sub2)) => format!("For want of a {0} the {1} was lost.\n", sub1, sub2),
        (sub1, None) => format!("And all for the want of a {0}.", sub1),
    }
}
