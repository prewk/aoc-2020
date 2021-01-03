use anyhow::{Result, anyhow, bail};

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

pub fn find_weakness(all: &[u64], invalid: u64) -> Result<u64> {
    for (i, a) in all.iter().enumerate() {
        let mut sum = *a;
        let mut smallest: Option<u64> = None;
        let mut largest: Option<u64> = None;
        for b in all.iter().skip(i + 1) {
            smallest = match smallest {
                None => Some(*b),
                Some(v) => {
                    if v < *b {
                        Some(v)
                    } else {
                        Some(*b)
                    }
                }
            };
            largest = match largest {
                None => Some(*b),
                Some(v) => {
                    if v > *b {
                        Some(v)
                    } else {
                        Some(*b)
                    }
                }
            };
            sum += *b;

            if sum == invalid {
                let sm = smallest.unwrap();
                let la = largest.unwrap();

                return Ok(sm + la);
            } else if sum > invalid {
                break;
            }
        }
    }

    bail!("Didn't find the sequence")
}