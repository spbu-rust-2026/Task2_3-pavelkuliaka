fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn reverse_words_at_odd_positions(text: &mut String) {
    let mut position = 0;
    let mut odd_position = true;
    let mut buffer = [0u8; 4];

    while position < text.len() {
        let Some(offset) = text[position..]
            .char_indices()
            .find(|(_, c)| !c.is_whitespace())
            .map(|(i, _)| i)
        else {
            break;
        };
        let start = position + offset;

        let end = match text[start..]
            .char_indices()
            .find(|(_, c)| c.is_whitespace())
        {
            Some((i, _)) => start + i,
            None => text.len(),
        };

        if odd_position {
            let mut right = end;
            while start < right {
                let head = text.remove(start);
                text.insert_str(right - head.len_utf8(), head.encode_utf8(&mut buffer));
                right -= head.len_utf8();
            }
        }

        odd_position = !odd_position;
        position = end;
    }
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
