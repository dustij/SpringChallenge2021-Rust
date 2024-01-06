use std::{ io, borrow::Borrow };

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap());
}

// =================================================================================================
// Cell
// =================================================================================================

// -------------------------------------------------
// Structs
// -------------------------------------------------

#[derive(Debug)]
struct Cell {
    index: i8,
    richness: i8,
    neighbors: Vec<i8>,
}

// =================================================================================================
// Tree
// =================================================================================================

// -------------------------------------------------
// Structs
// -------------------------------------------------

#[derive(Debug)]
struct Tree {
    cell_index: i8,
    size: u8,
    is_mine: bool,
    is_dormant: bool,
    is_shadowed_next_turn: bool,
}

// -------------------------------------------------
// Functions
// -------------------------------------------------

fn is_shadowed_next_turn(game_state: &GameState, size: u8, cell_index: i8) -> bool {
    fn check_neighbor(
        game_state: &GameState,
        root_tree_size: u8,
        sun_index: u8,
        cell_index: i8,
        count: u8
    ) -> bool {
        // Get reference to cell
        let cell = game_state.cells
            .iter()
            .find(|cell| cell.index == cell_index)
            .expect("Could not find cell when checking if shadowed");

        // Get cell index of neighbor
        let neighbor_index = cell.neighbors[sun_index as usize];

        if neighbor_index == -1 {
            // This neighbor does not exist (edge of map)
            return false;
        }

        // Get reference to neighbor tree
        let neighbor_tree = game_state.trees.iter().find(|tree| tree.cell_index == neighbor_index);

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
        check_neighbor(game_state, root_tree_size, sun_index, neighbor_index, count - 1)
    }

    // Index of sun direction
    let sun_index = (game_state.day + (1 % 6) + 3) % 6;

    // How many neighbors to check
    let count = size;

    // Start recursive check
    check_neighbor(game_state, size, sun_index, cell_index, count)
}

// =================================================================================================
// Action
// =================================================================================================

// -------------------------------------------------
// Structs
// -------------------------------------------------

#[derive(Debug)]
#[derive(PartialEq)]
enum ActionType {
    Wait,
    Seed,
    Grow,
    Complete,
}

#[derive(Debug)]
struct Action {
    action_type: ActionType,
    source_cell_index: i8,
    target_cell_index: i8,
    visits: u8,
}

// -------------------------------------------------
// Parsing
// -------------------------------------------------

fn parse_action_from_input(input: &str) -> Action {
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    let action_type = match parts[0] {
        "WAIT" => ActionType::Wait,
        "SEED" => ActionType::Seed,
        "GROW" => ActionType::Grow,
        "COMPLETE" => ActionType::Complete,
        _ => panic!("Invalid action type"),
    };

    let source_cell_index = match action_type {
        ActionType::Seed => parts[1].parse::<i8>().unwrap_or(-1),
        _ => -1,
    };

    let target_cell_index = match action_type {
        ActionType::Seed => parts[2].parse::<i8>().unwrap_or(-1),
        ActionType::Grow => parts[1].parse::<i8>().unwrap_or(-1),
        ActionType::Complete => parts[1].parse::<i8>().unwrap_or(-1),
        _ => -1,
    };

    Action {
        action_type,
        source_cell_index,
        target_cell_index,
        visits: 0,
    }
}

fn parse_action_to_output(action: &Action) -> String {
    match action.action_type {
        ActionType::Seed =>
            format!("SEED {} {}", action.source_cell_index, action.target_cell_index),
        ActionType::Grow => format!("GROW {}", action.target_cell_index),
        ActionType::Complete => format!("COMPLETE {}", action.target_cell_index),
        ActionType::Wait => String::from("WAIT"),
    }
}

// -------------------------------------------------
// Functions
// -------------------------------------------------

fn get_best_action(game_state: &GameState) -> &Action {
    // Choose random action (for now)
    let choosen_index = (game_state.day % (game_state.possible_actions.len() as u8)) as usize;

    // debug next sun score
    eprintln!("Next sun score: {}", get_new_sun_score(game_state, true));

    game_state.possible_actions[choosen_index].borrow()
}

// =================================================================================================
// GameState
// =================================================================================================

// -------------------------------------------------
// Structs
// -------------------------------------------------

#[derive(Debug)]
struct GameState {
    day: u8,
    nutrients: u8,
    my_sun: u32,
    my_score: u32,
    opp_sun: u32,
    opp_score: u32,
    opponent_is_waiting: bool,
    possible_actions: Vec<Action>,
    cells: Vec<Cell>,
    trees: Vec<Tree>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            day: 0,
            nutrients: 0,
            my_sun: 0,
            my_score: 0,
            opp_sun: 0,
            opp_score: 0,
            opponent_is_waiting: false,
            possible_actions: Vec::<Action>::new(),
            cells: Vec::<Cell>::new(),
            trees: Vec::<Tree>::new(),
        }
    }
}

// -------------------------------------------------
// Functions
// -------------------------------------------------

fn get_new_sun_score(game_state: &GameState, is_mine: bool) -> u32 {
    if is_mine {
        game_state.my_sun +
            game_state.trees
                .iter()
                .filter(|tree| tree.is_mine)
                .map(|tree| tree.size as u32)
                .sum::<u32>() -
            game_state.trees
                .iter()
                .filter(|tree| tree.is_mine && tree.is_shadowed_next_turn)
                .map(|tree| tree.size as u32)
                .sum::<u32>()
    } else {
        game_state.opp_sun +
            game_state.trees
                .iter()
                .filter(|tree| !tree.is_mine)
                .map(|tree| tree.size as u32)
                .sum::<u32>() -
            game_state.trees
                .iter()
                .filter(|tree| !tree.is_mine && tree.is_shadowed_next_turn)
                .map(|tree| tree.size as u32)
                .sum::<u32>()
    }
}

// =================================================================================================
// Main
// =================================================================================================

fn main() {
    let mut game_state = GameState::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37

    for _ in 0..number_of_cells as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let index = parse_input!(inputs[0], i8); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i8); // 0 if the cell is unusable, 1-3 for usable cells
        let neigh_0 = parse_input!(inputs[2], i8); // the index of the neighbouring cell for each direction
        let neigh_1 = parse_input!(inputs[3], i8);
        let neigh_2 = parse_input!(inputs[4], i8);
        let neigh_3 = parse_input!(inputs[5], i8);
        let neigh_4 = parse_input!(inputs[6], i8);
        let neigh_5 = parse_input!(inputs[7], i8);

        game_state.cells.push(Cell {
            index,
            richness,
            neighbors: vec![neigh_0, neigh_1, neigh_2, neigh_3, neigh_4, neigh_5],
        });
    }
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let day = parse_input!(input_line, u8); // the game lasts 24 days: 0-23

        game_state.day = day;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let nutrients = parse_input!(input_line, u8); // the base score you gain from the next COMPLETE action

        game_state.nutrients = nutrients;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let sun = parse_input!(inputs[0], u32); // your sun points
        let score = parse_input!(inputs[1], u32); // your current score

        game_state.my_sun = sun;
        game_state.my_score = score;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opp_sun = parse_input!(inputs[0], u32); // opponent's sun points
        let opp_score = parse_input!(inputs[1], u32); // opponent's score
        let opp_is_waiting = parse_input!(inputs[2], u8); // whether your opponent is asleep until the next day

        game_state.opp_sun = opp_sun;
        game_state.opp_score = opp_score;
        game_state.opponent_is_waiting = opp_is_waiting == 1;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_trees = parse_input!(input_line, i32); // the current amount of trees

        let mut trees = Vec::<Tree>::new();

        for _ in 0..number_of_trees {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let cell_index = parse_input!(inputs[0], i8); // location of this tree
            let size = parse_input!(inputs[1], u8); // size of this tree: 0-3
            let is_mine = parse_input!(inputs[2], u8); // 1 if this is your tree
            let is_dormant = parse_input!(inputs[3], u8); // 1 if this tree is dormant
            let is_shadowed = is_shadowed_next_turn(&game_state, size, cell_index);

            trees.push(Tree {
                cell_index,
                size,
                is_mine: is_mine == 1,
                is_dormant: is_dormant == 1,
                is_shadowed_next_turn: is_shadowed,
            });
        }

        game_state.trees = trees;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_possible_actions = parse_input!(input_line, i32); // all legal actions

        let mut possible_actions = Vec::<Action>::new();

        for _ in 0..number_of_possible_actions {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let input_line = input_line.trim_matches('\n').to_string();
            let possible_action = parse_action_from_input(&input_line);

            possible_actions.push(possible_action);
        }

        game_state.possible_actions = possible_actions;

        let choosen_action = get_best_action(&game_state);
        let action_ouput = parse_action_to_output(choosen_action);

        // Debugging output
        eprintln!("Game State:");
        eprintln!("Day: {}", game_state.day);
        eprintln!("Nutrients: {}", game_state.nutrients);
        eprintln!("My Sun: {}", game_state.my_sun);
        eprintln!("My Score: {}", game_state.my_score);
        eprintln!("Opponent Sun: {}", game_state.opp_sun);
        eprintln!("Opponent Score: {}", game_state.opp_score);
        eprintln!("Opponent is waiting: {}", game_state.opponent_is_waiting);
        eprintln!("Possible actions: {:?}", game_state.possible_actions);
        eprintln!("Trees: {:?}", game_state.trees);
        eprintln!("Cells: {:?}", game_state.cells);

        eprintln!("\nAction chosen: {}", action_ouput);

        // Action output
        println!("{}", action_ouput);
    }
}
