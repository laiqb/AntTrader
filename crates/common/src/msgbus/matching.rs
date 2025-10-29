use std::thread::panicking;

use super::{MStr, Pattern, Topic};

pub fn is_matching_backtracking(topic: MStr<Topic>, pattern: MStr<Pattern>) -> bool{
    let topic_bytes = topic.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    is_matching(topic_bytes, pattern_bytes)
}

#[must_use]
pub fn is_matching(topic: &[u8], pattern: &[u8]) -> bool{
    // Stack to store states for backtracking (topic_idx, pattern_idx)
    let mut stack = vec![(0, 0)];

    while let Some((mut i, mut j)) = stack.pop() {
        loop {
            // Found a match if we've consumed both strings
            if i == topic.len() && j == pattern.len() {
                return true;
            }

            // If we've reached the end of the pattern, break to try other paths
            if j == pattern.len() {
                break;
            }

            // Handle '*' wildcard
            if pattern[j] == b'*' {
                // Try skipping '*' entirely first
                stack.push((i, j + 1));

                // Continue with matching current character and keeping '*'
                if i < topic.len() {
                    i += 1;
                    continue;
                }
                break;
            }
            // Handle '?' or exact character match
            else if i < topic.len() && (pattern[j] == b'?' || topic[i] == pattern[j]) {
                // Continue matching linearly without stack operations
                i += 1;
                j += 1;
                continue;
            }

            // No match found in current path
            break;
        }
    }

    false
}