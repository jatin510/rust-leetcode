pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut total_money = 0;
    let mut change = vec![0, 0, 0];
    let mut result = true;

    bills.iter().for_each(|bill| {
        if *bill == 5 {
            total_money += 5;
            change[0] += 1;
        } else {
            let change_needed = bill - 5;

            if change_needed == 5 {
                if change[0] > 0 {
                    change[0] -= 1;
                    change[1] += 1;
                } else {
                    // mark as transaction failed
                    result = false;
                }
            } else if change_needed == 15 {
                if change[1] > 0 && change[0] > 0 {
                    change[1] -= 1;
                    change[0] -= 1;
                    change[2] += 1;
                } else if change[0] >= 3 {
                    change[0] -= 3;
                    change[2] += 1;
                } else {
                    // mark as transaction failed
                    result = false;
                }
            }
        }
    });

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let bills = vec![5, 5, 5, 10, 20];
        assert_eq!(lemonade_change(bills), true);
    }

    #[test]
    fn test_2() {
        let bills = vec![5, 5, 10];
        assert_eq!(lemonade_change(bills), true);
    }

    #[test]
    fn test_3() {
        let bills = vec![10, 10];
        assert_eq!(lemonade_change(bills), false);
    }

    #[test]
    fn test_4() {
        let bills = vec![5, 5, 10, 10, 20];
        assert_eq!(lemonade_change(bills), false);
    }
}