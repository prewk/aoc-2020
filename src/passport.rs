#[derive(PartialOrd, PartialEq, Debug)]
pub struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<u16>,
}

impl Passport {
    pub fn from_line(line: &str) -> Passport {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;

        line
            .split(' ')
            .for_each(|entry| {
                let parts: Vec<&str> = entry.split(':').collect();

                match parts.get(0) {
                    Some(&"byr") => { birth_year = Some(parts.get(1).unwrap().parse::<u16>().unwrap()); },
                    Some(&"iyr") => { issue_year = Some(parts.get(1).unwrap().parse::<u16>().unwrap()); },
                    Some(&"eyr") => { expiration_year = Some(parts.get(1).unwrap().parse::<u16>().unwrap()); },
                    Some(&"hgt") => { height = Some(parts.get(1).unwrap().to_string()); },
                    Some(&"hcl") => { hair_color = Some(parts.get(1).unwrap().to_string()); },
                    Some(&"ecl") => { eye_color = Some(parts.get(1).unwrap().to_string()); },
                    Some(&"pid") => { passport_id = Some(parts.get(1).unwrap().to_string()); },
                    Some(&"cid") => { country_id = Some(parts.get(1).unwrap().parse::<u16>().unwrap()); },
                    _ => panic!("WTF"),
                }
            });

        Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        }
    }

    pub fn validate(&self) -> bool {
        self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.height.is_some() &&
            self.hair_color.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_creation() {
        let p = Passport::from_line(&"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm");

        assert_eq!(p, Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some("183cm".to_string()),
            hair_color: Some("#fffffd".to_string()),
            eye_color: Some("gry".to_string()),
            passport_id: Some("860033327".to_string()),
            country_id: Some(147),
        });

        assert!(p.validate());
    }
}
