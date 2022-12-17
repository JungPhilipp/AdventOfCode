use std::{
    cmp::{max, min},
    collections::HashMap,
};

use itertools::Itertools;
use log::info;
use regex::Regex;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 15:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}
type Input = HashMap<Room, (Flow, Vec<Room>)>;

fn parse(input: &str) -> Input {
    let expression =
        Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();
    expression
        .captures_iter(input)
        .map(|line| {
            let name = to_name(&line[1]);
            let flow = line[2].parse::<Flow>().expect("Flow should be integer");
            let neighbors = line[3]
                .split(',')
                .map(|name| to_name(name.trim()))
                .collect_vec();
            (name, (flow, neighbors))
        })
        .collect()
}
type Cost = usize;
type Graph = HashMap<Room, (Flow, HashMap<Room, Cost>)>;

fn cleanup_graph(graph: Input) -> Graph {
    let mut graph_with_costs = graph
        .into_iter()
        .map(|(key, value)| {
            let new_value = (
                value.0,
                value.1.into_iter().map(|neighbor| (neighbor, 1)).collect(),
            );
            (key, new_value)
        })
        .collect::<Graph>();

    while let Some((room, (_, neighbors))) = graph_with_costs
        .iter()
        .find(|(_, value)| value.0 == 0)
        .map(|(key, value)| (*key, value.clone()))
    {
        for (neighbor, costs) in neighbors.iter() {
            for (other_neighbor, other_costs) in neighbors.iter() {
                if let Some((_, neighbors)) = graph_with_costs.get_mut(neighbor) {
                    let new_costs = neighbors.entry(*other_neighbor).or_insert(Cost::MAX);
                    *new_costs = min(*new_costs, *costs + *other_costs);
                }
            }
        }
        graph_with_costs.remove(&room);
    }

    graph_with_costs
}

type Flow = usize;

fn to_name(name: &str) -> Room {
    name.chars().collect_tuple().expect("Should be two chars")
}
type Room = (char, char);
type Time = i64;

fn max_flow(
    graph: &HashMap<Room, (Flow, Vec<Room>)>,
    room: Room,
    time: Time,
    opened: &Vec<Room>,
    cache: &mut HashMap<(Room, Time, Vec<Room>), Flow>,
) -> usize {
    if time <= 0 || opened.len() == graph.len() {
        return 0;
    }
    let cache_key = (room, time, opened.clone());
    if let Some(flow) = cache.get(&cache_key) {
        return *flow;
    }
    let (flow_rate, neighbors) = graph.get(&room).unwrap();

    let without_open = neighbors
        .iter()
        .map(|neighbor| max_flow(graph, *neighbor, time - 1, opened, cache))
        .max()
        .unwrap();

    let next_opened = opened
        .iter()
        .cloned()
        .chain([room].into_iter())
        .sorted()
        .collect_vec();

    let with_open = if *flow_rate > 0 && !opened.contains(&room) {
        (time - 1) as usize * flow_rate + max_flow(graph, room, time - 1, &next_opened, cache)
    } else {
        0
    };

    let flow = max(without_open, with_open);
    cache.insert(cache_key, flow);
    flow
}

fn solve_part1(input: Input) -> usize {
    max_flow(&input, ('A', 'A'), 30, &vec![], &mut HashMap::new())
}

fn max_flow_vec(
    graph: &Graph,
    rooms: (Room, Room),
    time: Time,
    opened: &Vec<Room>,
    cache: &mut HashMap<((Room, Room), Time, Vec<Room>), Flow>,
) -> usize {
    if time <= 0 || opened.len() == graph.len() {
        return 0;
    }
    let cache_key = (rooms, time, opened.clone());
    if let Some(flow) = cache.get(&cache_key) {
        return *flow;
    }

    let mut flow = 0;
    let (flow_rate_1, neighbors_1) = graph.get(&rooms.0).unwrap();
    let (flow_rate_2, neighbors_2) = graph.get(&rooms.1).unwrap();

    for (neighbor_1, _) in neighbors_1.iter()
    /*.chain([(rooms.0, 1)].iter()) */
    {
        if *neighbor_1 == rooms.0 && (*flow_rate_1 == 0 || opened.contains(&rooms.0)) {
            continue;
        }

        for (neighbor_2, _) in neighbors_2.iter()
        /*.chain([rooms.1].iter()) */
        {
            let mut flow_inner = 0;
            let mut opened_new = opened.clone();
            if *neighbor_1 == rooms.0 {
                opened_new.push(rooms.0);
                flow_inner += (time - 1) as usize * flow_rate_1;
            }
            if *neighbor_2 == rooms.1 {
                if *flow_rate_2 == 0 || opened_new.contains(&rooms.1) {
                    continue;
                }
                opened_new.push(rooms.1);
                flow_inner += (time - 1) as usize * flow_rate_2;
            }
            flow_inner += max_flow_vec(
                graph,
                [*neighbor_1, *neighbor_2]
                    .into_iter()
                    .sorted()
                    .collect_tuple()
                    .unwrap(),
                time - 1,
                &opened_new,
                cache,
            );
            flow = max(flow, flow_inner);
        }
    }

    cache.insert(cache_key, flow);
    flow
}

fn solve_part2(input: Input) -> usize {
    max_flow_vec(
        &cleanup_graph(input),
        (('A', 'A'), ('A', 'A')),
        26,
        &vec![],
        &mut HashMap::new(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    macro_rules! EXAMPLE_PATH {
        () => {
            "example_1.txt"
        };
    }

    #[test]
    fn example_1() {
        let input = parse(include_str!(EXAMPLE_PATH!()));
        assert_eq!(solve_part1(input), 1651);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 2330);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 1707);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
