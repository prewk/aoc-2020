use anyhow::{Result, anyhow};

pub fn test_number(all: &[u64], subject_index: usize) -> Result<bool> {
    if subject_index < 25 {
        return Err(anyhow!("Subject before preamble"));
    }

    if subject_index >= all.len() {
        return Err(anyhow!("Subject out of bounds"));
    }

    let subject = all[subject_index];

    let test_against: Vec<u64> = all.iter().skip(subject_index - 25).take(25).map(|v| *v).collect();

    for (i, a) in test_against.iter().enumerate() {
        for (ii, b) in test_against.iter().enumerate() {
            if i != ii && a + *b == subject {
                return Ok(true)
            }
        }
    }

    Ok(false)
}
