use std::{time::Instant, collections::{HashSet, BinaryHeap, HashMap}, usize, cmp::Ordering};

use crate::{util::read_file, direction::*};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    position: (usize, usize),
    last_from: Direction,
    moves_from: usize,
    cost_so_far: usize,
    parent: ((usize, usize), Direction, usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost_so_far.cmp(&self.cost_so_far)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day17_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day17.txt");
    }
    let file = read_file(&filename_l);

    let time = Instant::now();
    let mut cost_map: Vec<Vec<usize>> = Vec::new();
    for line in file.lines() {
        let mut line_vec: Vec<usize> = Vec::new();
        for character in line.chars() {
            let value: usize = String::from(character).parse().unwrap();
            line_vec.push(value);
        }
        cost_map.push(line_vec.clone());
    }

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    let mut closed: HashMap<((usize, usize), Direction, usize), (usize, ((usize, usize), Direction, usize))> = HashMap::new();
    cost_map[0][0] = 0;
    open.push(State {position: (0,0), cost_so_far: 0, last_from: Direction::Left, moves_from: 0, parent: ((0, 0), Direction::Left, 0)});
    while !open.is_empty() {
        // get next node to process
        let state_g = open.pop().unwrap();
        let done = closed.get(&(state_g.position, state_g.last_from, state_g.moves_from));
        if done != None {
            let (cost_so_far, _) = done.unwrap();
            if cost_so_far <= &state_g.cost_so_far {
                continue;
            }
        }

        closed.insert((state_g.position, state_g.last_from, state_g.moves_from), (state_g.cost_so_far, state_g.parent));
        if closed.len() % 10000 == 0 {
            println!("curr closed len: {}, curr node {:?}", closed.len(), state_g)
        }
        if state_g.position == (cost_map.len()-1, cost_map[state_g.position.0].len()-1) {
            break;
        }

        let mut successors: Vec<State> = get_successors(&cost_map, state_g);
        for succ_state in &mut successors {
            succ_state.parent = (state_g.position, state_g.last_from, state_g.moves_from);
            open.push(*succ_state);
        }
    }

    let result = closed.iter().filter(|(k, _)| k.0.0 == cost_map.len()-1 && k.0.1 == cost_map.len() - 1).min_by_key(|(_, v)| v.0).unwrap().1.0;

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn get_successors(cost_map: &Vec<Vec<usize>>, state: State) -> Vec<State> {
    let mut successors: Vec<State> = Vec::new();
    let mut allowed_directions: HashSet<Direction> = HashSet::from([Direction::Bottom, Direction::Left, Direction::Right, Direction::Top]);
    allowed_directions.remove(&state.last_from);
    if state.moves_from >= 3 {
        allowed_directions.remove(&state.last_from.get_opposite());
    }
    if state.position.0 == 0 {
        allowed_directions.remove(&Direction::Top);
    }
    if state.position.1 == 0 {
        allowed_directions.remove(&Direction::Left);
    }
    if state.position.0 == cost_map.len() - 1 {
        allowed_directions.remove(&Direction::Bottom);
    }
    if state.position.1 == cost_map[state.position.0].len() - 1 {
        allowed_directions.remove(&Direction::Right);
    }
    for dir in allowed_directions {
        let new_position = dir.move_position(state.position);
        let new_moves_from = if dir.get_opposite() != state.last_from { 1 } else { state.moves_from + 1 };
        successors.push(State { position: new_position, last_from: dir.get_opposite(), moves_from: new_moves_from, cost_so_far: state.cost_so_far + cost_map[new_position.0][new_position.1], parent: (state.position, state.last_from, state.moves_from) });
    }
    successors
}

pub fn day17_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day17.txt");
    }
    let file = read_file(&filename_l);

    let time = Instant::now();
    let mut cost_map: Vec<Vec<usize>> = Vec::new();
    for line in file.lines() {
        let mut line_vec: Vec<usize> = Vec::new();
        for character in line.chars() {
            let value: usize = String::from(character).parse().unwrap();
            line_vec.push(value);
        }
        cost_map.push(line_vec.clone());
    }

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    let mut closed: HashMap<((usize, usize), Direction, usize), (usize, ((usize, usize), Direction, usize))> = HashMap::new();
    cost_map[0][0] = 0;
    open.push(State {position: (0,0), cost_so_far: 0, last_from: Direction::Left, moves_from: 0, parent: ((0, 0), Direction::Left, 0)});
    while !open.is_empty() {
        // get next node to process
        let state_g = open.pop().unwrap();
        let done = closed.get(&(state_g.position, state_g.last_from, state_g.moves_from));
        if done != None {
            let (cost_so_far, _) = done.unwrap();
            if cost_so_far <= &state_g.cost_so_far {
                continue;
            }
        }

        closed.insert((state_g.position, state_g.last_from, state_g.moves_from), (state_g.cost_so_far, state_g.parent));
        if closed.len() % 10000 == 0 {
            println!("curr closed len: {}, curr node {:?}", closed.len(), state_g)
        }
        // if state_g.position == (cost_map.len()-1, cost_map[state_g.position.0].len()-1) && state_g.moves_from >= 4 {
        //     println!("exited with break: {:?}", state_g.position);
        //     break;
        // }

        let mut successors: Vec<State> = get_successors_2(&cost_map, state_g);
        for succ_state in &mut successors {
            succ_state.parent = (state_g.position, state_g.last_from, state_g.moves_from);
            open.push(*succ_state);
        }
    }

    let finalists = closed.iter().filter(|(k, _)| k.0.0 == cost_map.len()-1 && k.0.1 == cost_map[k.0.0].len() - 1 && k.2 >= 4);
    let the_best = finalists.min_by_key(|(_, v)| v.0).unwrap();
    let result = the_best.1.0;

    let mut curr_node = the_best;
    let mut path: Vec<(usize, usize)> = Vec::new();
    while curr_node.0.0.0 != 0 || curr_node.0.0.1 != 0 {
        path.push(curr_node.0.0);
        let parent = curr_node.1.1;
        curr_node = closed.get_key_value(&parent).unwrap();
    }
    // for dir in path.iter().rev() {
    //     println!("{:?}", dir);
    // }
    for (i, line) in cost_map.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("X");
            } else {
                print!("{}", val);
            }
        }
        println!();
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn get_successors_2(cost_map: &Vec<Vec<usize>>, state: State) -> Vec<State> {
    let mut successors: Vec<State> = Vec::new();
    let mut allowed_directions: HashSet<Direction> = HashSet::from([Direction::Bottom, Direction::Left, Direction::Right, Direction::Top]);
    allowed_directions.remove(&state.last_from);
    if state.moves_from < 4 && state.position != (0, 0) {
        allowed_directions.clear();
        allowed_directions.insert(state.last_from.get_opposite());
    }
    if state.moves_from >= 10 {
        allowed_directions.remove(&state.last_from.get_opposite());
    }
    if state.position.0 == 0 {
        allowed_directions.remove(&Direction::Top);
    }
    if state.position.1 == 0 {
        allowed_directions.remove(&Direction::Left);
    }
    if state.position.0 == cost_map.len() - 1 {
        allowed_directions.remove(&Direction::Bottom);
    }
    if state.position.1 == cost_map[state.position.0].len() - 1 {
        allowed_directions.remove(&Direction::Right);
    }
    for dir in allowed_directions {
        let new_position = dir.move_position(state.position);
        let new_moves_from = if dir.get_opposite() != state.last_from { 1 } else { state.moves_from + 1 };
        successors.push(State { position: new_position, last_from: dir.get_opposite(), moves_from: new_moves_from, cost_so_far: state.cost_so_far + cost_map[new_position.0][new_position.1], parent: (state.position, state.last_from, state.moves_from) });
    }
    successors
}