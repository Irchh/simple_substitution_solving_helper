use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io;
use std::io::Write;

// TODO: Take optional alphabet as argument
fn index_of_coincidence(s: String) -> f64 {
    let N = number_of_alphabetic_chars(s.clone()) as f64;
    let freq = char_sums(s.clone());
    let mut sum = 0f64;
    for c in 'a'..='z' {
        if freq.contains_key(&c) {
            let f = *freq.get(&c).unwrap() as f64;
            sum += f*(f-1f64);
        }
    }
    sum/(N*(N-1f64))
}

fn number_of_alphabetic_chars(s: String) -> usize {
    let mut n = 0;
    s.chars().for_each(|c|{
        if c.is_alphabetic() {
            n += 1;
        }
    });
    n
}

fn char_sums(s: String) -> BTreeMap<char, usize> {
    let mut sums = BTreeMap::<char, usize>::new();
    for c in s.to_lowercase().chars() {
        if !c.is_alphabetic() {
            continue
        }
        if let Some(f) = sums.get_mut(&c) {
            *f += 1;
        } else {
            sums.insert(c, 1);
        }
    }
    sums
}

fn char_frequencies(s: String) -> BTreeMap<char, f64> {
    let mut sums = char_sums(s.clone());
    let mut freq = BTreeMap::<char, f64>::new();
    for (c, sum) in sums {
        freq.insert(c, sum as f64/number_of_alphabetic_chars(s.clone()) as f64);
    }
    freq
}

fn main() {
    let mut cipher_text = String::new();
    print!("Enter cipher text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cipher_text).unwrap();
    println!();
    let cipher_length = number_of_alphabetic_chars(cipher_text.clone());
    let coincidence = index_of_coincidence(cipher_text.clone());
    let freq = char_frequencies(cipher_text.clone());
    let mut freq_vec = freq.iter().map(|(&c, &freq)|{
        (c, freq)
    }).collect::<Vec<(char, f64)>>();
    freq_vec.sort_by(|&(_, freq_l), &(_, freq_r)|{
        if freq_l < freq_r {
            Ordering::Greater
        } else if freq_l > freq_r {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    println!("Cipher-text len: {}", cipher_length);
    println!("Cipher-text index of coincidence: {}", coincidence);
    println!("{:#?}", freq_vec);

    let mut char_map = BTreeMap::<char, char>::new();
    let mut buffer = String::new();
    loop {
        let mut colored_text = String::new();
        let modified_cipher_text = cipher_text.clone().chars().map(|c| {
            if char_map.contains_key(&c) {
                let ch = *char_map.get(&c).unwrap();
                if ch == c { // If a char is mapped to itself, show it in green as to not confuse user.
                    colored_text += format!("\x1b[92m{ch}\x1b[0m").as_str();
                } else { // If a char is mapped to a different char show it as red to signify change.
                    colored_text += format!("\x1b[91m{ch}\x1b[0m").as_str();
                }
                ch
            } else {
                colored_text.push(c);
                c
            }
        }).collect::<Vec<char>>();
        println!("\n-Current text: {}", colored_text);
        print!("Enter substitution: ");
        io::stdout().flush().unwrap();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        println!();
        let mut command = buffer.clone().chars().collect::<Vec<char>>();
        if command.last().is_some() && command.last().unwrap() == &'\n' {
            command.pop();
        }
        if command.len() == 3 {
            // Map character to character
            let cipher_char = *command.get(0).unwrap();
            let operator_char = *command.get(1).unwrap();
            let mapped_to_char = *command.get(2).unwrap();
            if operator_char == '=' {
                if cipher_char.is_ascii() && mapped_to_char.is_ascii() {
                    char_map.insert(cipher_char, mapped_to_char);
                } else {
                    println!("Not ascii, try again.");
                }
            } else {
                println!("Unknown command {:?}", operator_char);
            }
        } else {
            match buffer.strip_suffix("\n").unwrap_or("") {
                "exit" => break,
                "mapped" => {
                    for (i, (orig, map)) in char_map.iter().enumerate() {
                        if i == 0 {
                            print!("{}: {}", orig, map)
                        } else {
                            print!(", {}: {}", orig, map)
                        }
                    }
                    println!()
                }
                cmd => {
                    println!("Unknown command {:?}", cmd);
                }
            }
        }
    }
}
