use anyhow::{Result, bail};
use regex::Regex;

trait Validate {
    fn validate(&self) -> bool;
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Year {
    pub value: u16,
    min: u16,
    max: u16
}

impl Year {
    pub fn birth_year_from_str(year: &str) -> Result<Year> {
        Ok(Year {
            value: year.parse::<u16>()?,
            min: 1920,
            max: 2002,
        })
    }

    pub fn issue_year_from_str(year: &str) -> Result<Year> {
        Ok(Year {
            value: year.parse::<u16>()?,
            min: 2010,
            max: 2020,
        })
    }

    pub fn exp_year_from_str(year: &str) -> Result<Year> {
        Ok(Year {
            value: year.parse::<u16>()?,
            min: 2020,
            max: 2030,
        })
    }
}

impl Validate for Year {
    fn validate(&self) -> bool {
        self.value >= self.min && self.value <= self.max
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Height {
    Cm(u8),
    Inch(u8),
    Unknown,
}

impl From<&str> for Height {
    fn from(height: &str) -> Self {
        if height.ends_with(&"cm") {
            Height::Cm(height[0..height.len() - 2].parse::<u8>().unwrap())
        } else if height.ends_with(&"in") {
            Height::Inch(height[0..height.len() - 2].parse::<u8>().unwrap())
        } else {
            Height::Unknown
        }
    }
}

impl Validate for Height {
    fn validate(&self) -> bool {
        match self {
            Height::Cm(v) => v >= &150u8 && v <= &193u8,
            Height::Inch(v) => v >= &59u8 && v <= &76u8,
            Height::Unknown => false,
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct HairColor {
    color: String,
}

impl From<&str> for HairColor {
    fn from(hair_color: &str) -> HairColor {
        HairColor { color: hair_color.to_string() }
    }
}

impl Validate for HairColor {
    fn validate(&self) -> bool {
        let re = Regex::new("^#[0-9a-f]{6}$").unwrap();

        re.is_match(&self.color)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct EyeColor {
    color: String,
}

impl From<&str> for EyeColor {
    fn from(color: &str) -> EyeColor {
        EyeColor { color: color.to_string() }
    }
}

impl Validate for EyeColor {
    fn validate(&self) -> bool {
        match &self.color.as_str() {
            &"amb" => true,
            &"blu" => true,
            &"brn" => true,
            &"gry" => true,
            &"grn" => true,
            &"hzl" => true,
            &"oth" => true,
            _ => false,
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
struct PassportId {
    id: String,
}

impl From<&str> for PassportId {
    fn from(id: &str) -> PassportId {
        PassportId { id: id.to_string() }
    }
}

impl Validate for PassportId {
    fn validate(&self) -> bool {
        let re = Regex::new("^\\d{9}$").unwrap();

        re.is_match(&self.id)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
struct CountryId {
    id: String,
}

impl From<&str> for CountryId {
    fn from(id: &str) -> CountryId {
        CountryId { id: id.to_string() }
    }
}

impl Validate for CountryId {
    fn validate(&self) -> bool {
        true
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Passport {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Height>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    passport_id: Option<PassportId>,
    country_id: Option<CountryId>,
}

impl Passport {
    pub fn from_line(line: &str) -> Result<Passport> {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;

        for entry in line.split(' ') {
            let parts: Vec<&str> = entry.split(':').collect();
            let part1 = *parts.get(1).unwrap();

            match parts.get(0) {
                Some(&"byr") => { birth_year = Some(Year::birth_year_from_str(part1)?); },
                Some(&"iyr") => { issue_year = Some(Year::issue_year_from_str(part1)?); },
                Some(&"eyr") => { expiration_year = Some(Year::exp_year_from_str(part1)?); },
                Some(&"hgt") => { height = Some(Height::from(part1)); },
                Some(&"hcl") => { hair_color = Some(HairColor::from(part1)); },
                Some(&"ecl") => { eye_color = Some(EyeColor::from(part1)); },
                Some(&"pid") => { passport_id = Some(PassportId::from(part1)); },
                Some(&"cid") => { country_id = Some(CountryId::from(part1)); },
                _ => {
                    bail!("Invalid property: {}", part1);
                },
            }
        }

        Ok(Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        })
    }

    pub fn validate_according_to_first_part(&self) -> bool {
        self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.height.is_some() &&
            self.hair_color.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some()
    }

    pub fn validate_according_to_second_art(&self) -> bool {
        self.birth_year.is_some() && self.birth_year.as_ref().unwrap().validate() &&
            self.issue_year.is_some() && self.issue_year.as_ref().unwrap().validate() &&
            self.expiration_year.is_some() && self.expiration_year.as_ref().unwrap().validate() &&
            self.height.is_some() && self.height.as_ref().unwrap().validate() &&
            self.hair_color.is_some() && self.hair_color.as_ref().unwrap().validate() &&
            self.eye_color.is_some() && self.eye_color.as_ref().unwrap().validate() &&
            self.passport_id.is_some() && self.passport_id.as_ref().unwrap().validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_creation() {
        let p = Passport::from_line(&"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm").unwrap();

        assert_eq!(p, Passport {
            birth_year: Some(Year::birth_year_from_str(&"1937").unwrap()),
            issue_year: Some(Year::issue_year_from_str(&"2017").unwrap()),
            expiration_year: Some(Year::exp_year_from_str(&"2020").unwrap()),
            height: Some(Height::from("183cm")),
            hair_color: Some(HairColor::from("#fffffd")),
            eye_color: Some(EyeColor::from("gry")),
            passport_id: Some(PassportId::from("860033327")),
            country_id: Some(CountryId::from("147")),
        });

        assert!(p.validate_according_to_first_part());
    }
}
