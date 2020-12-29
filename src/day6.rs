use crate::customs::Group;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Group> {
    let mut lines = vec![];
    let mut groups = vec![];

    for line in input.lines() {
        if line.trim().len() == 0 {
            groups.push(Group::from(lines.join("\n")));
            lines = vec![];
        }

        lines.push(line);
    }
    groups.push(Group::from(lines.join("\n")));

    groups
}

#[aoc(day6, part1)]
pub fn part1(groups: &[Group]) -> u64 {
    groups
        .iter()
        .fold(0, |acc, g| acc + g.count_yes_questions())
}

#[aoc(day6, part2)]
pub fn part2(groups: &[Group]) -> u64 {
    groups
        .iter()
        .fold(0, |acc, g| {
            acc + g.count_all_yes_questions()
        })
}
