
pub fn optimized_levenshtein(a: &[u8], b: &[u8]) -> usize {
    let len_a = a.len();
    let len_b = b.len();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a ;
    }

    let mut prev_row = (0..=len_b).collect::<Vec<usize>>();
    let mut curr_row = vec![0; (len_b + 1).try_into().unwrap()];

    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr_row[j] = *[
                prev_row[j] + 1,          // deletion
                curr_row[j - 1] + 1,      // insertion
                prev_row[j - 1] + cost,   // substitution
            ].iter().min().unwrap();
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[len_b]
}