use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

const RE: &str = r#"^(\S+) \S+ \S+ \[([^\]]+)\] "(\S+) (\S+) \S+" (\d+)"#;

fn main() {
    let regex = Regex::new(RE).unwrap();
    let mut html_requests = HashMap::new();

    // Take the lock once so read_line() doesn't constantly acquire/release it
    let mut stdin = io::stdin().lock();

    // Reusable buffer reduces the number of memory allocations
    let mut input = String::new();

    loop {
        input.clear(); // Clear the string but keep the capacity

        let Ok(bytes_read) = stdin.read_line(&mut input) else {
            break;
        };

        if bytes_read == 0 {
            break;
        }

        let captures = match regex.captures(&input) {
            Some(captures) => captures,
            None => continue,
        };

        let method = &captures[3];
        let url = &captures[4];
        let status_code: u32 = captures[5].parse().unwrap_or(0);

        if status_code == 200 && method == "GET" && url.ends_with(".html") {
            match html_requests.get_mut(url) {
                Some(count) => *count += 1,
                None => {
                    html_requests.insert(url.to_string(), 1);
                }
            }
        }
    }

    // into_iter() - moves data out of the HashMap instead of copying it
    // collect()   - returns a vector of tuples (URL, count)
    let mut pairs: Vec<_> = html_requests.into_iter().collect();

    // Use unstable sort for better performance
    pairs.sort_unstable_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (url, count) in pairs {
        println!("{} {}", count, url);
    }
}
