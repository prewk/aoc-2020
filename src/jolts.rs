use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn find_jolt_differences(adapters: &Vec<u64>) -> Result<HashMap<u64, u64>> {
    let mut diffs = HashMap::new();

    for (i, jolt) in adapters.iter().enumerate() {
        if let Some(next) = adapters.get(i + 1) {
            let diff = next - jolt;

            if let Some(_) = diffs.get(&diff) {
                *diffs.get_mut(&diff).context("This can't happen")? += 1;
            } else {
                diffs.insert(diff, 1);
            }
        }
    }

    Ok(diffs)
}

pub fn try_chain(adapters: &Vec<u64>) -> bool {
    let mut last_adapter = adapters[0];
    for adapter in adapters.iter().skip(1) {
        if *adapter - last_adapter > 3 {
            return false;
        }

        last_adapter = *adapter;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut jolts = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22];
        jolts.sort();

        let map = find_jolt_differences(&jolts).unwrap();

        assert_eq!(map[&1], 7);
        assert_eq!(map[&3], 5);
    }

    #[test]
    fn test_example2() {
        let mut jolts = vec![
            0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
            35, 8, 17, 7, 9, 4, 2, 34, 10, 3, 52,
        ];
        jolts.sort();

        let map = find_jolt_differences(&jolts).unwrap();

        assert_eq!(map[&1], 22);
        assert_eq!(map[&3], 10);
    }

    fn test_chain() {
        let mut jolts = vec![
            0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
            35, 8, 17, 7, 9, 4, 2, 34, 10, 3, 52,
        ];
        jolts.sort();

        assert!(try_chain(&jolts));
    }
}
