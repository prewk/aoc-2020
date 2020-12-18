pub fn find_2020_2_sum_and_mul(entries: &[u64]) -> Option<u64> {
    for (i, e) in entries.iter().enumerate() {
        for (ii, ee) in entries.iter().enumerate() {
            if i != ii && e + ee == 2020 {
                return Some(e * ee);
            }
        }
    }

    return None;
}

pub fn find_2020_3_sum_and_mul(entries: &[u64]) -> Option<u64> {
    for (i, e) in entries.iter().enumerate() {
        for (ii, ee) in entries.iter().enumerate() {
            for (iii, eee) in entries.iter().enumerate() {
                if i != ii && i != iii && ii != iii && e + ee + eee == 2020 {
                    return Some(e * ee * eee);
                }
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let sum = find_2020_2_sum_and_mul(&vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ]);

        assert_eq!(sum, Some(514579));
    }

    #[test]
    fn example2() {
        let sum = find_2020_3_sum_and_mul(&vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ]);

        assert_eq!(sum, Some(241861950));
    }
}
