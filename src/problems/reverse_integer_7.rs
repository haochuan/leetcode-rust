// https://leetcode.com/problems/reverse-integer/

#[allow(dead_code, unused_variables)]
fn reverse(x: i32) -> i32 {
    let mut chars: Vec<_> = x.to_string().chars().collect();
    chars.reverse();

    let sign = chars.pop().unwrap();
    let is_negative = match sign {
        '+' => false,
        '-' => true,
        _ => {
            chars.push(sign);
            false
        }
    };

    let s: String = chars.into_iter().collect();
    let result: i32 = match s.parse() {
        Ok(x) => x,
        Err(e) => 0, // return 0 is overflow
    };

    match is_negative {
        true => result * -1,
        false => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(reverse(123), 321);
    }
    #[test]
    fn test2() {
        assert_eq!(reverse(-123), -321);
    }
    #[test]
    fn test3() {
        assert_eq!(reverse(120), 21);
    }
}
