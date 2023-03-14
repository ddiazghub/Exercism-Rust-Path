pub fn translate(input: &str) -> String {
    if input.len() == 0 {
        return String::new();
    }

    let mut translated = String::new();

    for word in input.split(' ') {
        let mut cluster = Vec::new();

        for ch in word.chars() {
            let cluster_len = cluster.len();

            match ch {
                _ if &cluster[..] == &['x', 'r'] || &cluster[..] == &['y', 't'] => {
                    cluster.clear();
                    break;
                },
                'u' if cluster_len > 0 && cluster[cluster_len - 1] == 'q' => {
                    cluster.push('u');
                    break;
                },
                'a'|'e'|'i'|'o'|'u' => break,
                'y' if cluster_len > 0 => break,
                _ => cluster.push(ch)
            }
        }

        translated.push_str(&word[cluster.len()..]);
        translated.push_str(&cluster.into_iter().collect::<String>());
        translated.push_str("ay ");
    }
    
    translated.pop();

    translated
}
