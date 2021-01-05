use std::collections::HashMap;
use anyhow::{Result, Context};

pub fn find_jolt_differences(adapters: &mut Vec<u64>) -> Result<HashMap<u64, u64>> {
    adapters.sort();
    adapters.insert(0, 0);

    let max = *adapters.iter().max().context("Found no max")?;
    let mut diffs = HashMap::new();
    adapters.push(max + 3);

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

fn simplify_adapters(adapters: &Vec<u64>) -> Result<Option<Vec64>> {
    let mut simplified = vec![];

    for (i, adapter) in adapters.iter().enumerate() {

    }

    match simplified.len() == adapters.len() {
        true => Ok(None),
        false => Ok(Some(simplified)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut jolts = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ];

        let map = find_jolt_differences(&mut jolts).unwrap();

        assert_eq!(map[&1], 7);
        assert_eq!(map[&3], 5);
    }

    #[test]
    fn test_example2() {
        let mut jolts = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ];

        let map = find_jolt_differences(&mut jolts).unwrap();

        assert_eq!(map[&1], 22);
        assert_eq!(map[&3], 10);
    }
}