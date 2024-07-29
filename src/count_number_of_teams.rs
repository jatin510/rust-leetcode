pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for i in 0..rating.len() {
        for j in i + 1..rating.len() {
            for k in j + 1..rating.len() {
                if (rating[i] < rating[j] && rating[j] < rating[k]) || (rating[i] > rating[j] && rating[j] > rating[k]) {
                    result += 1;
                }
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let rating = vec![2, 5, 3, 4, 1];
        assert_eq!(num_teams(rating), 3);
    }

    #[test]
    fn test_2() {
        let rating = vec![2, 1, 3];
        assert_eq!(num_teams(rating), 0);
    }

    #[test]
    fn test_3() {
        let rating = vec![1, 2, 3, 4];
        assert_eq!(num_teams(rating), 4);
    }
}