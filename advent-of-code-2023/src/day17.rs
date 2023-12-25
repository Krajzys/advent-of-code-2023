use std::{time::Instant, collections::{HashSet, BinaryHeap, HashMap}, usize, cmp::Ordering};

use crate::util::read_file;

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    position: (usize, usize),
    last_from: Direction,
    moves_from: usize,
    cost_so_far: usize,
    parent: usize
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

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Top => Direction::Bottom
        }
    }

    pub fn move_position(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Bottom => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
            Direction::Top => (position.0 - 1, position.1)
        }
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
    let mut closed: HashMap<((usize, usize), Direction, usize), (usize, usize)> = HashMap::new();
    cost_map[0][0] = 0;
    open.push(State {position: (0,0), cost_so_far: 0, last_from: Direction::Left, moves_from: 0, parent: 0});
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
            succ_state.parent = closed.len() - 1;
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
        successors.push(State { position: new_position, last_from: dir.get_opposite(), moves_from: new_moves_from, cost_so_far: state.cost_so_far + cost_map[new_position.0][new_position.1], parent: 0 });
    }
    successors
}

pub fn day17_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day17.txt");
    }   
    let file = read_file(&filename_l);
    
    let result = 0;
    let time = Instant::now();
    let mut tile_map: Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {
        tile_map.push(String::from(line));
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}