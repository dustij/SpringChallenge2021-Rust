use std::io;
use std::fmt;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap());
}

// ================================================================================================
// Area
// ================================================================================================

#[derive(Clone, Copy)]
struct Cell {
    index: i32,
    richness: i32,
    neighbors_ids: [i32; 6],
}

impl Cell {
    fn new() -> Self {
        Cell { index: 0, richness: 0, neighbors_ids: [-1; 6] }
    }
}

type Area = [Cell; 37];

fn get_area() -> Area {
    let mut area = [Cell::new(); 37];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37
    for i in 0..number_of_cells as usize {
        let mut input_line = String::new();
        let mut neighbors_ids = [-1; 6];
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let index = parse_input!(inputs[0], i32); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
        for i in 2..8 {
            neighbors_ids[i - 2] = parse_input!(inputs[i], i32);
        }
        area[i] = Cell { index, richness, neighbors_ids };
    }

    area
}

// ================================================================================================
// Forest
// ================================================================================================

struct Tree {
    cell_index: i32,
    size: i32,
    is_mine: bool,
    is_dormant: bool,
    is_shadowed: bool,
}

type Forest = Vec<Tree>;

fn get_forest() -> Forest {
    let mut forest = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
    for _ in 0..number_of_trees as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let cell_index = parse_input!(inputs[0], i32); // location of this tree
        let size = parse_input!(inputs[1], i32); // size of this tree: 0-3
        let is_mine = parse_input!(inputs[2], i32) == 1; // 1 if this is your tree
        let is_dormant = parse_input!(inputs[3], i32) == 1; // 1 if this tree is dormant
        let is_shadowed = false;
        forest.push(Tree { cell_index, size, is_mine, is_dormant, is_shadowed });
    }
    forest
}

fn get_is_shadowed(context: &GameContext, size: i32, cell_index: i32) -> bool {
    fn check_neighbor(
        context: &GameContext,
        root_tree_size: i32,
        sun_index: i32,
        cell_index: i32,
        count: i32
    ) -> bool {
        // Get reference to cell
        let cell = context.area
            .iter()
            .find(|cell| cell.index == cell_index)
            .expect("Could not find cell when checking if shadowed");

        // Get cell index of neighbor
        let neighbor_index = cell.neighbors_ids[sun_index as usize];

        if neighbor_index == -1 {
            // This neighbor does not exist (edge of map)
            return false;
        }

        // Get reference to neighbor tree
        let neighbor_tree = context.forest.iter().find(|tree| tree.cell_index == neighbor_index);

        // Check if there is a neighbor tree and if it is bigger than my tree
        if let Some(found_tree) = neighbor_tree {
            if found_tree.size >= root_tree_size {
                return true;
            }
        }

        // Check if we should continue checking neighbors
        if count == 1 {
            // We have checked the last neighbor and none are casting a shadow
            return false;
        }

        // Continue checking neighbors
        check_neighbor(context, root_tree_size, sun_index, neighbor_index, count - 1)
    }

    // Index of sun direction
    let sun_index = ((context.day % 6) + 3) % 6;

    // How many neighbors to check
    let count = size;

    // Start recursive check
    check_neighbor(context, size, sun_index, cell_index, count)
}

// ================================================================================================
// Action
// ================================================================================================

enum Action {
    Grow(i32),
    Seed(i32, i32),
    Complete(i32),
    Wait,
}

impl From<&String> for Action {
    fn from(s: &String) -> Self {
        let inputs = s.split(" ").collect::<Vec<_>>();
        match inputs[0] {
            "GROW" => Action::Grow(parse_input!(inputs[1], i32)),
            "SEED" => Action::Seed(parse_input!(inputs[1], i32), parse_input!(inputs[2], i32)),
            "COMPLETE" => Action::Complete(parse_input!(inputs[1], i32)),
            "WAIT" => Action::Wait,
            _ => {
                panic!("Wrong action input");
            }
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Action::Grow(i) => format!("GROW {}", i),
            Action::Seed(i, j) => format!("SEED {} {}", i, j),
            Action::Complete(i) => format!("COMPLETE {}", i),
            Action::Wait => String::from("WAIT"),
        })
    }
}

type ActionList = Vec<Action>;

fn get_actionlist() -> ActionList {
    let mut action_list = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_possible_actions = parse_input!(input_line, i32);
    for _ in 0..number_of_possible_actions as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let possible_action = input_line.trim_matches('\n').to_string();
        action_list.push(Action::from(&possible_action));
    }
    action_list
}

// ================================================================================================
// GameContext
// ================================================================================================

struct GameContext {
    day: i32,
    nutrients: i32,
    sun: i32,
    score: i32,
    op_sun: i32,
    op_score: i32,
    op_is_waiting: bool,
    area: Area,
    forest: Forest,
    action_list: ActionList,
}

fn get_game_context(area: Area) -> GameContext {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let sun = parse_input!(inputs[0], i32); // your sun points
    let score = parse_input!(inputs[1], i32); // your current score
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let op_sun = parse_input!(inputs[0], i32); // opponent's sun points
    let op_score = parse_input!(inputs[1], i32); // opponent's score
    let op_is_waiting = parse_input!(inputs[2], i32) == 1; // whether your opponent is asleep until the next day

    GameContext {
        day,
        nutrients,
        sun,
        score,
        op_sun,
        op_score,
        op_is_waiting,
        area,
        forest: get_forest(),
        action_list: get_actionlist(),
    }
}

// ================================================================================================
// Main
// ================================================================================================

fn main() {
    let area = get_area();

    // game loop
    loop {
        let answer = String::from("WAIT");

        let context = get_game_context(area);

        for a in context.action_list.iter() {
            eprintln!("Action : {}", a);
        }

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("{}", answer);
    }
}
