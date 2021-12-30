use std::collections::HashMap;

type Link = ((String, String), u32);

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Link> {
    input
        .lines()
        .map(|line| {
            // AlphaCentauri to Snowdin = 66
            let (locations, distance) = line.split_once(" = ").unwrap();
            let (left, right) = locations.split_once(" to ").unwrap();
            let distance = distance.parse().unwrap();
            ((left.to_string(), right.to_string()), distance)
        })
        .collect()
}

type LinkMap = HashMap<String, Vec<(String, u32)>>;

fn to_link_map(links: &[Link]) -> LinkMap {
    let mut map = LinkMap::new();
    for ((start, end), distance) in links {
        map.entry(start.clone())
            .or_default()
            .push((end.clone(), *distance));
        map.entry(end.clone())
            .or_default()
            .push((start.clone(), *distance));
    }
    map
}

fn solve(prefix: Vec<String>, link_map: &LinkMap, longest: bool) -> (Vec<String>, u32) {
    let mut best = None;
    let next_links = match prefix.last() {
        Some(last) => {
            // Start from last location in current path, with given costs
            link_map.get(last).unwrap().clone()
        }
        None => {
            // Start from any location, with cost 0
            link_map.keys().map(|start| (start.clone(), 0)).collect()
        }
    };
    for (next, distance) in next_links {
        if prefix.contains(&next) {
            continue;
        }
        let mut path = prefix.clone();
        path.push(next.clone());
        let (next_path, next_distance) = solve(path, link_map, longest);
        let next_distance = distance + next_distance;
        best = match best {
            Some((old_path, old_distance))
                if (!longest && next_distance >= old_distance)
                    || (longest && next_distance <= old_distance) =>
            {
                Some((old_path, old_distance))
            }
            _ => Some((next_path, next_distance)),
        }
    }
    best.unwrap_or((prefix, 0))
}

#[aoc(day9, part1)]
pub fn part1(links: &[Link]) -> u32 {
    let link_map = to_link_map(links);
    let (_, distance) = solve(vec![], &link_map, false);
    distance
}

#[aoc(day9, part2)]
pub fn part2(links: &[Link]) -> u32 {
    let link_map = to_link_map(links);
    let (_, distance) = solve(vec![], &link_map, true);
    distance
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 605);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 982);
    }
}
