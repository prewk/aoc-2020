use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn input_str_to_hash_map(input: &str) -> HashMap<String, HashMap<String, u64>> {
    let mut output: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let bag_re = Regex::new("^(\\d)+").unwrap();

    for line in input.lines() {
        let name_content: Vec<&str> = line.split(" contain ").collect();
        let name = name_content.get(0).unwrap().to_string().replace(".", "").replace("bags", "bag");
        let content = name_content.get(1).unwrap().to_string();

        if content == "no other bags." {
            output.insert(name, HashMap::new());
        } else {
            for dependee in content.split(", ") {
                let captures = bag_re.captures(dependee).unwrap();

                let dependee_count_str = captures.get(0).unwrap().as_str();
                let dependee_count = dependee_count_str.parse::<u64>().unwrap();
                let dependee_name = dependee[dependee_count_str.len() + 1..].to_string().replace(".", "").replace("bags", "bag");

                if let Some(map) = output.get_mut(&name) {
                    map.insert(dependee_name.clone(), dependee_count);
                } else {
                    let mut map = HashMap::new();
                    map.insert(dependee_name.clone(), dependee_count);

                    output.insert(name.clone(), map);
                }
            }
        }
    }

    output
}

pub fn find_carrier_bags(bags: &HashMap<String, HashMap<String, u64>>, query: &str) -> HashSet<String> {
    let mut output = HashSet::new();

    for (name, content) in bags {
        // Find all bags that contain <query>
        if content.contains_key(query) {
            // This bag contains <query>
            output.insert(name.clone());

            // Find all bags that contain this bag
            for deeper in find_carrier_bags(bags, name) {
                output.insert(deeper.clone());
            }
        }
    }

    output
}

pub fn find_content_count(bags: &HashMap<String, HashMap<String, u64>>, query: &str, start: u64) -> u64 {
    let mut all = start;
    let inners = bags.get(query).unwrap();

    for (name, count) in inners {
        all += count * find_content_count(bags, name, 1);
    }

    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_carrier_bags() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                           dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                           bright white bags contain 1 shiny gold bag.\n\
                           muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                           shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                           dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                           vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                           faded blue bags contain no other bags.\n\
                           dotted black bags contain no other bags.";

        let bags = input_str_to_hash_map(input);

        assert_eq!(find_carrier_bags(&bags, "shiny gold bag").len(), 4);
    }

    #[test]
    fn test_find_content_count() {
        let input = "shiny gold bags contain 2 dark red bags.\n\
                           dark red bags contain 2 dark orange bags.\n\
                           dark orange bags contain 2 dark yellow bags.\n\
                           dark yellow bags contain 2 dark green bags.\n\
                           dark green bags contain 2 dark blue bags.\n\
                           dark blue bags contain 2 dark violet bags.\n\
                           dark violet bags contain no other bags.";

        let bags = input_str_to_hash_map(input);

        assert_eq!(find_content_count(&bags, "shiny gold bag", 0), 126);
    }
}