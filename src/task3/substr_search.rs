use std::collections::HashMap;

pub fn boyer_moor_search(string: &str, template: &str, case_sensitive: bool) -> Vec<usize> {
    let mut last_occ: HashMap<char, usize> = HashMap::new();
    let string_vec: Vec<char>;
    let template_vec: Vec<char>;

    if case_sensitive {
        string_vec = string.chars().collect();
        template_vec = template.chars().collect();
    } else {
        string_vec = string.to_lowercase().chars().collect();
        template_vec = template.to_lowercase().chars().collect();
    }

    template_vec.iter().enumerate().for_each(|(i, char)| {
        last_occ.insert(*char, i);
    });
    let mut result: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut j: usize;
    while i <= (string_vec.len() - template_vec.len()) {
        j = i + template_vec.len();
        if let Some(k) = find_dif(&string_vec[i..j], &template_vec, template_vec.len() - 1) {
            let abs_idx = i + k;
            match last_occ.get(&string_vec[abs_idx]) {
                None => i = abs_idx + 1,
                Some(&l) if k < l => i = abs_idx + 1,
                Some(&l) => i = abs_idx - l,
            }
        } else {
            result.push(i);
            i += 1;
        }
    }
    result
}

fn find_dif(s: &[char], t: &[char], mut i: usize) -> Option<usize> {
    while s[i] == t[i] {
        if i == 0 {
            return None;
        }
        i -= 1;
    }
    Some(i)
}
