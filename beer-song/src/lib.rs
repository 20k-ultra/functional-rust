pub fn verse(n: u32) -> String {
    match n {
      0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
      _ => format!("{0} {1} of beer on the wall, {0} {1} of beer.\nTake {2} down and pass it around, {3} {4} of beer on the wall.\n",
       n, bottle_or_bottles(n), one_or_it(n), number_or_no_more(if n == 0 { 0 } else { n - 1 }), bottle_or_bottles(if n == 0 { 0 } else { n - 1 }))
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    let mut position = start;
    // Loop threw the rounds of the song
    while position != if end == 0 { 0 } else { end - 1 } {
        song.push_str(&verse(position));
        song.push_str("\n");
        position -= 1;
    }
    // Check if we want to finish the song
    if position == 0 && end == 0 {
        song.push_str(&verse(position));
        song.push_str("\n");
    }
    // Trim last newline
    song.truncate(if song.is_empty() { 0 } else { song.len() - 1 });
    // Return song
    song
}

fn bottle_or_bottles(n: u32) -> String {
    if n > 1 || n == 0 {
        return String::from("bottles");
    }
    String::from("bottle")
}

fn one_or_it(n: u32) -> String {
    if n > 1 {
        return String::from("one");
    }
    String::from("it")
}

fn number_or_no_more(n: u32) -> String {
    if n > 0 {
        return n.to_string();
    }
    String::from("no more")
}
