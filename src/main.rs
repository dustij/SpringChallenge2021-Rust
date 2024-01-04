use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap());
}

#[derive(Debug)]
struct GameState {
    day: u8,
    nutrients: u8,
    cells: Vec<Cell>,
    possible_actions: Vec<Action>,
    trees: Vec<Tree>,
    my_sun: u8,
    my_score: u32,
    opp_sun: u8,
    opp_score: u32,
    opponent_is_waiting: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            day: 0,
            nutrients: 0,
            cells: Vec::new(),
            possible_actions: Vec::new(),
            trees: Vec::new(),
            my_sun: 0,
            my_score: 0,
            opp_sun: 0,
            opp_score: 0,
            opponent_is_waiting: false,
        }
    }
}

#[derive(Debug)]
struct Cell {
    index: u8,
    richness: u8,
    neighbors: Vec<u8>,
}

#[derive(Debug)]
struct Tree {
    cell_index: u8,
    size: u8,
    is_mine: bool,
    is_dormant: bool,
}

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
    source_cell_index: Option<u8>,
    target_cell_index: Option<u8>,
}

fn parse_line_to_action(input: &str) -> Action {
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    let action_type = match parts[0] {
        "WAIT" => ActionType::Wait,
        "SEED" => ActionType::Seed,
        "GROW" => ActionType::Grow,
        "COMPLETE" => ActionType::Complete,
        _ => panic!("Invalid action type"),
    };

    let source_cell_index = match action_type {
        ActionType::Seed => Some(parts[1].parse::<u8>().unwrap()),
        _ => None,
    };

    let target_cell_index = match action_type {
        ActionType::Seed => Some(parts[2].parse::<u8>().unwrap()),
        ActionType::Wait => None,
        _ => Some(parts[1].parse::<u8>().unwrap()),
    };

    Action {
        action_type,
        source_cell_index,
        target_cell_index,
    }
}

fn parse_action_to_line(action: &Action) -> String {
    match action.action_type {
        ActionType::Wait => "WAIT".to_string(),
        ActionType::Seed =>
            format!(
                "SEED {} {}",
                action.source_cell_index.unwrap(),
                action.target_cell_index.unwrap()
            ),
        ActionType::Grow => format!("GROW {}", action.target_cell_index.unwrap()),
        ActionType::Complete => format!("COMPLETE {}", action.target_cell_index.unwrap()),
    }
}

// =================================================================================================
// =================================================================================================

// ------------------------------------------------------------------
// Action Logic
// ------------------------------------------------------------------

fn choose_action(game_state: &GameState) -> &Action {
    let eval = evaluate_state(game_state);
    eprintln!("Eval: {}", eval);

    // Simple decsisions for now
    if is_action_possible(&game_state.possible_actions, ActionType::Complete) {
        return game_state.possible_actions
            .iter()
            .find(|action| action.action_type == ActionType::Complete)
            .unwrap();
    } else if is_action_possible(&game_state.possible_actions, ActionType::Grow) {
        return game_state.possible_actions
            .iter()
            .find(|action| action.action_type == ActionType::Grow)
            .unwrap();
    } else if is_action_possible(&game_state.possible_actions, ActionType::Seed) {
        return game_state.possible_actions
            .iter()
            .find(|action| action.action_type == ActionType::Seed)
            .unwrap();
    } else if is_action_possible(&game_state.possible_actions, ActionType::Wait) {
        return game_state.possible_actions
            .iter()
            .find(|action| action.action_type == ActionType::Wait)
            .unwrap();
    } else {
        panic!("Something went wrong while choosing an action");
    }
}

// ------------------------------------------------------------------
//
// ------------------------------------------------------------------

fn is_action_possible(actions: &[Action], action_type: ActionType) -> bool {
    actions.iter().any(|action| action.action_type == action_type)
}

fn evaluate_state(game_state: &GameState) -> f32 {
    let my_score = (game_state.my_score as f32) + (game_state.my_sun as f32) / 3.0;
    let opponent_score = (game_state.opp_score as f32) + (game_state.opp_sun as f32) / 3.0;

    if my_score > opponent_score {
        let diff = my_score - opponent_score;

        if diff > 5.0 {
            return 1.0 + (diff - 5.0) * 0.001;
        } else {
            return 0.5 + (0.5 * diff) / 5.0;
        }
    } else if my_score < opponent_score {
        let diff = opponent_score - my_score;

        if diff > 5.0 {
            return -1.0 - (diff - 5.0) * 0.001;
        } else {
            return -0.5 - (0.5 * diff) / 5.0;
        }
    } else {
        let my_tree_count = game_state.trees
            .iter()
            .filter(|tree| tree.is_mine)
            .collect::<Vec<&Tree>>()
            .len() as u8;

        let opponent_tree_count = game_state.trees
            .iter()
            .filter(|tree| !tree.is_mine)
            .collect::<Vec<&Tree>>()
            .len() as u8;

        if my_tree_count > opponent_tree_count {
            return 0.25 + my_score * 0.001;
        } else if my_tree_count < opponent_tree_count {
            return -0.25 + my_score * 0.001;
        } else {
            return my_score * 0.001;
        }
    }
}

// =================================================================================================
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

        let index = parse_input!(inputs[0], i32); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
        let neigh_0 = parse_input!(inputs[2], i32); // the index of the neighbouring cell for each direction
        let neigh_1 = parse_input!(inputs[3], i32);
        let neigh_2 = parse_input!(inputs[4], i32);
        let neigh_3 = parse_input!(inputs[5], i32);
        let neigh_4 = parse_input!(inputs[6], i32);
        let neigh_5 = parse_input!(inputs[7], i32);

        game_state.cells.push(Cell {
            index: index as u8,
            richness: richness as u8,
            neighbors: vec![
                neigh_0 as u8,
                neigh_1 as u8,
                neigh_2 as u8,
                neigh_3 as u8,
                neigh_4 as u8,
                neigh_5 as u8
            ],
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
        let sun = parse_input!(inputs[0], u8); // your sun points
        let score = parse_input!(inputs[1], u32); // your current score

        game_state.my_sun = sun;
        game_state.my_score = score;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opp_sun = parse_input!(inputs[0], u8); // opponent's sun points
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
            let cell_index = parse_input!(inputs[0], u8); // location of this tree
            let size = parse_input!(inputs[1], u8); // size of this tree: 0-3
            let is_mine = parse_input!(inputs[2], u8); // 1 if this is your tree
            let is_dormant = parse_input!(inputs[3], u8); // 1 if this tree is dormant

            trees.push(Tree {
                cell_index,
                size,
                is_mine: is_mine == 1,
                is_dormant: is_dormant == 1,
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
            let possible_action = parse_line_to_action(&input_line);

            possible_actions.push(possible_action);
        }

        game_state.possible_actions = possible_actions;

        let choosen_action = choose_action(&game_state);
        let action_line = parse_action_to_line(&choosen_action);

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

        eprintln!("\nAction chosen: {}", action_line);

        // Action output
        println!("{}", action_line); // GROW cell_index | SEED source_index target_index | COMPLETE cell_index | WAIT <message>
    }
}
