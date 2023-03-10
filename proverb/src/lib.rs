pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = list.windows(2)
        .fold(String::new(), |mut proverb, pair| {
            proverb.push_str("For want of a ");
            proverb.push_str(pair[0]);
            proverb.push_str(" the ");
            proverb.push_str(pair[1]);
            proverb.push_str(" was lost.\n");

            proverb
        });

    if list.len() > 0 {
        proverb.push_str("And all for the want of a ");
        proverb.push_str(list[0]);
        proverb.push('.');
    }

    proverb
}
