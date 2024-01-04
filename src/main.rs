use rand::Rng;

fn get_symbols() -> [(char, Vec<&'static str>); 8] {
    [
        ('s', vec![
            "ach", "ack", "ad", "age", "ald", "ale", "an", "ang", "ar", "ard",
            "as", "ash", "at", "ath", "augh", "aw", "ban", "bel", "bur", "cer",
            "cha", "che", "dan", "dar", "del", "den", "dra", "dyn", "ech", "eld",
            "elm", "em", "en", "end", "eng", "enth", "er", "ess", "est", "et",
            "gar", "gha", "hat", "hin", "hon", "ia", "ight", "ild", "im", "ina",
            "ine", "ing", "ir", "is", "iss", "it", "kal", "kel", "kim", "kin",
            "ler", "lor", "lye", "mor", "mos", "nal", "ny", "nys", "old", "om",
            "on", "or", "orm", "os", "ough", "per", "pol", "qua", "que", "rad",
            "rak", "ran", "ray", "ril", "ris", "rod", "roth", "ryn", "sam",
            "say", "ser", "shy", "skel", "sul", "tai", "tan", "tas", "ther",
            "tia", "tin", "ton", "tor", "tur", "um", "und", "unt", "urn", "usk",
            "ust", "ver", "ves", "vor", "war", "wor", "yer"
        ]),
        ('v', vec!["a", "e", "i", "o", "u", "y"]),
        ('d', vec![
            "à", "è", "ì", "ò", "ù", "ỳ", 
            "á", "é", "í", "ó", "ú", "ý", 
            "â", "ê", "î", "ô", "û", "ŷ", 
            "ä", "ë", "ï", "ö", "ü", "ÿ", 
            "å", "ø"
        ]),
        ('D', vec![
            "ć", "ç", "þ", "ñ", "ń", "ǹ", "ṕ",
            "ð", "ŕ", "ĺ", "ś", "ź", "ẁ", "ẃ"
        ]),
        ('V', vec![
            "a", "e", "i", "o", "u", "y", "ae", "ai", "au", "ay", "ea", "ee",
    		"ei", "eu", "ey", "ia", "ie", "oe", "oi", "oo", "ou", "ui"
        ]),
        ('c', vec![
            "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r",
            "s", "t", "v", "w", "x", "y", "z"
        ]),
        ('B', vec![
            "b", "bl", "br", "c", "ch", "chr", "chth", "cl", "cr", "ct", "d", "dr",
            "f", "g", "h", "j", "k", "kh", "khr", "l", "ll", "m", "mn", "n", "p", "ph", 
            "qu", "r", "rh", "s", "sch", "sh", "sl", "sm", "sn", "st", "str", "sw", "t", 
            "th", "thr", "tr", "v", "w", "wh", "y", "z", "zh"
        ]),
        ('C', vec![
            "b", "c", "ch", "ck", "d", "f", "g", "gh", "h", "k", "kh", "l", "ld", "ll",
            "lt", "m", "n", "nd", "nn", "nt", "p", "ph", "q", "r", "rd", "rr",
            "rt", "s", "sh", "ss", "st", "t", "th", "v", "vr", "w", "y", "z"
        ]),
    ]
}

fn generate_random_template() -> String {
    let mut template = String::new();
    let mut rand = rand::thread_rng();
    let symbols = get_symbols();
    let symbol_number = rand.gen_range(2..15);
    for _ in 0..symbol_number {
        template += &symbols[rand.gen_range(0..symbols.len())].0.to_string();
    }

    template
}

fn generate_name(template: String) -> String {
    let mut rand = rand::thread_rng();
    let symbols = get_symbols();

    let mut name = String::new();
        let mut litteral = false;
        let mut litteral_string = String::new();
        let mut symbol = false;
        let mut symbol_string = String::new();
        for char in template.clone().chars() {
            if char == '<' {
                symbol = true;
                continue;
            } else if char == '>' {
                if symbol_string.contains('|') {
                    let possible_string = symbol_string.split('|').collect::<Vec<&str>>();
                    name += &possible_string[rand.gen_range(0..possible_string.len())];
                } else {
                    name += &symbol_string;
                }

                symbol = false;
                symbol_string = String::new();
                continue;
            }

            if !symbol {
                if char == '(' {
                    litteral = true;
                    continue;
                } else if char == ')' {
                    if litteral_string.contains('|') {
                        let possible_string = litteral_string.split('|').collect::<Vec<&str>>();
                        name += &possible_string[rand.gen_range(0..possible_string.len())];
                    } else {
                        name += &litteral_string;
                    }

                    litteral = false;
                    litteral_string = String::new();
                    continue;
                }
            }

            if !litteral && !symbol {
                if char == '\'' || char == '-' {
                    name += &char.to_string();
                } else {
                    for replacement in symbols.clone() {
                        if replacement.0 == char && !litteral {
                            name += &replacement.1[rand.gen_range(0..replacement.1.len())].to_string();
                        }
                    }
                }
            } else if litteral {
                litteral_string += &char.to_string();
            } else if symbol {
                symbol_string += &char.to_string();
            }
        }

        let mut name_chars = name.chars();
        name = match name_chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + name_chars.as_str(),
        };
        dbg!(&name);

        name
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut input = String::new();

    if args.len() == 2 {
        input = args[1].clone();
    }
    if input == "".to_string() {
        input = generate_random_template();
    }

    dbg!(&input);

    for _ in 0..10 {
        generate_name(input.clone());
    }
}
