use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

const DAY: Day = 23;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let pairs = parse(&input);
    let sets = find_sets_with_t(&pairs);
    sets.len()
}

fn solve_part_2(input: impl Lines) -> String {
    let clusters = find_clusters(&parse(&input));
    let longest = clusters.iter().max_by_key(|c| c.len()).unwrap();
    longest.iter().sorted().join(",")
}

fn find_sets_with_t(connections: &[[SmolStr; 2]]) -> Vec<HashSet<SmolStr>> {
    let mapping = get_mapping(connections);

    mapping
        .keys()
        .cloned()
        .combinations(3)
        .filter(|t| {
            t.iter().any(|item| item.starts_with("t"))
                && t.iter().all(|item| {
                    t.iter()
                        .all(|other_item| other_item == item || mapping[item].contains(other_item))
                })
        })
        .map(HashSet::from_iter)
        .collect()
}

fn find_clusters(connections: &[[SmolStr; 2]]) -> Vec<HashSet<SmolStr>> {
    let mappings = get_mapping(connections);
    let mut sets: Vec<HashSet<SmolStr>> = vec![];

    for (item, connected) in mappings {
        // add to any fully connected existing cluster
        for set in &mut sets {
            if set.iter().all(|k| connected.contains(k)) {
                set.insert(item.clone());
            }
        }

        // create a new potential cluster
        sets.push(HashSet::from([item]));
    }

    sets
}

fn get_mapping(connections: &[[SmolStr; 2]]) -> HashMap<SmolStr, HashSet<SmolStr>> {
    let mapping = connections
        .iter()
        .cloned()
        .flat_map(|[a, b]| [(a.clone(), b.clone()), (b, a)])
        .into_grouping_map()
        .fold(HashSet::new(), |mut acc, _key, v| {
            acc.insert(v);
            acc
        });
    mapping
}

fn parse(input: &impl Lines) -> Vec<[SmolStr; 2]> {
    input
        .get_lines()
        .filter_map(|l| {
            l.split("-")
                .map(SmolStr::from)
                .collect::<Vec<_>>()
                .try_into()
                .ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 7, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, "co,de,ka,ta", TEST_INPUT);
    }
}
