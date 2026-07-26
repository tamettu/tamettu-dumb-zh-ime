use std::{
    collections::BTreeMap,
    env,
    io::{Write, stdin, stdout},
};
mod data;
fn input(tips: &str) -> String {
    loop {
        print!("\x1b[32m::{tips}\x1b[0m> ");
        stdout().flush().expect("cant show tips. fk nvidia");
        let mut usr_input = String::new();
        stdin()
            .read_line(&mut usr_input)
            .expect("\x1b[31mU must using Chrome, ur pc is out of ram\x1b[0m");
        if usr_input.trim().is_empty() {
            println!("\x1b[31mplease enter things!\x1b[0m]");
            continue;
        } else {
            return usr_input.trim().to_string();
        };
    }
}
fn zip_input(text: &str) -> Option<BTreeMap<char, i8>> {
    let mut new_text_map: BTreeMap<char, i8> = BTreeMap::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == 'F' {
            i += 1;
            if i >= text.len() {
                new_text_map.insert('F', 2);
                continue;
            }
            if let Some(c_digit) = chars[i].to_digit(10) {
                if let Ok(c_i8) = c_digit.try_into() {
                    new_text_map.insert('F', c_i8);
                    i += 1;
                }
            } else {
                new_text_map.insert('F', 2);
            }
            continue;
        }
        if chars[i].is_ascii_lowercase() {
            *new_text_map.entry(chars[i]).or_insert(0) += 1;
            i += 1;
            continue;
        }
        if !chars[i].is_ascii_digit() {
            i += 1;
            continue;
        } else if i + 1 >= chars.len() || chars[i + 1].is_ascii_digit() {
            println!("\x1b[31msry, can't handle it\x1b[0m");
            return None;
        }
        let amount = match chars[i].to_digit(10) {
            Some(a) => a as i8,
            _ => {
                println!("\x1b[31mit shouldnt happen, maybe some ass wrote special number\x1b[0m");
                return None;
            }
        };
        if amount <= 0 {
            i += 2;
            continue;
        }
        i += 1;
        if i >= chars.len() {
            return Some(new_text_map);
        }
        if chars[i].is_ascii_lowercase() {
            *new_text_map.entry(chars[i]).or_insert(0) += amount;
            i += 1;
            continue;
        } else {
            while i < chars.len() && chars[i].is_ascii_uppercase() && chars[i] != 'F' {
                *new_text_map
                    .entry(chars[i].to_ascii_lowercase())
                    .or_insert(0) += amount;
                i += 1;
            }
        }
    }
    Some(new_text_map)
}
fn replenish_0_char(all_char: &[char], text: &mut BTreeMap<char, i8>) {
    for &a in all_char {
        text.entry(a).or_insert(0);
    }
}
fn find_words(
    all_char: &[char],
    text: &mut BTreeMap<char, i8>,
    data: &[(BTreeMap<char, i8>, Vec<String>)],
    data_len: &[i8],
) -> Option<Vec<String>> {
    let mut all_words: Vec<String> = Vec::new();
    if text.get(&'F').is_none() {
        let (_, v) = data.iter().find(|&(k, _)| k == text)?;
        all_words.extend(v.clone());
        return Some(all_words);
    }
    let Some(max_diff) = text.get(&'F').copied() else {
        panic!("\x1b[31mur pc 100% broken!!!\x1b[0m")
    };
    text.remove(&'F');
    replenish_0_char(all_char, text);
    let mut text_len: i8 = 0;
    for (_, &v) in text.iter() {
        text_len += v;
    }
    println!("{}", max_diff);
    for (i, (k, v)) in data.iter().enumerate() {
        if data_len[i].abs_diff(text_len) >= max_diff as u8 {
            continue;
        }
        let mut is_pass = true;
        for (k2, &v2) in k.iter() {
            match text.get(k2) {
                Some(a) if a.abs_diff(v2) < 2 && !['e', 'r', 'a', 's'].contains(k2) => continue,

                Some(a) if *a == v2 => continue,

                Some(_) => {
                    is_pass = false;
                    continue;
                }
                _ => println!("\x1b[31mfk micoslop\x1b[0m"),
            }
        }
        if !is_pass {
            continue;
        }
        if all_words.contains(&v[0]) {
            continue;
        }
        all_words.extend(v.clone());
    }
    Some(all_words)
}
fn main() {
    let all_char: [char; 8] = ['q', 'w', 'e', 'r', 't', 'y', 'a', 's'];
    let data = data::reverse_map();
    let mut data_length: Vec<i8> = Vec::new();
    for (k, _) in &data {
        let mut total: i8 = 0;
        for (_, &v) in k.iter() {
            total += v;
        }
        data_length.push(total);
    }
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let raw_input = &args[1];
        if let Some(mut text) = zip_input(raw_input) {
            if let Some(list_of_text) = find_words(&all_char, &mut text, &data, &data_length) {
                for c in list_of_text {
                    println!("{}", c); // 印出每行一個候選字給 Wofi 讀取
                }
            }
        }
        return; // 處理完參數就結束程式
    }
    loop {
        let Some(mut text) = zip_input(&input(" ")) else {
            continue;
        };
        let Some(list_of_text) = find_words(&all_char, &mut text, &data, &data_length) else {
            continue;
        };
        for c in list_of_text {
            println!("{}", c);
        }
    }
}
