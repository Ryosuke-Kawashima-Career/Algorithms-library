use proconio::{input, marker::Chars};
// abc414C
// Bit全探索(O(N2^N))
fn main() {
    input!{t: usize}
    let mut answer: Vec<String> = Vec::new();
    for _case in 0..t {
        // n: medichines, s[state_i: 0indexed]: if s[state_i]==1, then state_i is dangerous.
        // if is_state_safe=0001, then state3(3=b011, medichine 0 and 1) is dangerous.
        input!{n: usize, s: Chars}
        let mut bits: usize = 1 << n;
        // 0: safe, 1: dangerous
        let mut is_state_safe: Vec<char> = vec!['0'];
        is_state_safe.extend_from_slice(&s);
        let mut is_state_ok: Vec<bool> = vec![false; bits];
        is_state_ok[0] = true;

        for state_bit in 0..bits {
            // current_state: ok -> next_state: ok
            if !is_state_ok[state_bit] {
                continue;
            }

            for medichine in 0..n {
                if state_bit >> medichine & 1 == 1 {
                    continue;
                }
                let next_state: usize = state_bit | (1 << medichine);
                // if the state is safe
                if is_state_safe[next_state] == '0' {
                    is_state_ok[next_state] = true;
                }
            }
        }

        if is_state_ok[bits-1] {
            answer.push("Yes".to_string());
        } else {
            answer.push("No".to_string());
        }
    }

    for ans in answer {
        println!("{}", ans);
    }
}