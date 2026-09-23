impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if  s.len() != t.len() {
            return false;
        }

        let mut alphabet = [0; 26];
        
        for c in s.chars() {
            let index = c as usize - 'a' as usize;
            alphabet[index] += 1;
        }

        for c in t.chars() {
            let index = c as usize - 'a' as usize;
            let value = &mut alphabet[index];
            if *value == 0 {
                return false;
            } else {
                *value -= 1;
            }
        }

        alphabet.iter().all(|&e| e == 0) // just true would be okay too
    }
}
