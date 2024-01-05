use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap());
}

#[derive(Debug)]
#[derive(Clone)]
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
#[derive(Clone)]
struct Cell {
    index: i8,
    richness: u8,
    neighbors: Vec<i8>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Tree {
    cell_index: i8,
    size: u8,
    is_mine: bool,
    is_dormant: bool,
    is_shadowed: bool,
}

fn is_shadowed(game_state: &GameState, cell_index: i8) -> bool {
    let tree = game_state.trees.iter().find(|tree| tree.cell_index == cell_index);

    match tree {
        Some(tree) => {
            let count = tree.size;
            let tree_size = tree.size;

            fn check_neighbor(
                game_state: &GameState,
                cell_index: i8,
                count: u8,
                tree_size: u8
            ) -> bool {
                let sun_direction = ((game_state.day % 6) + 3) % 6;
                let cell = game_state.cells
                    .iter()
                    .find(|cell| cell.index == cell_index)
                    .cloned()
                    .unwrap_or(Cell {
                        index: -1,
                        richness: 0,
                        neighbors: vec![-1, -1, -1, -1, -1, -1],
                    });
                let neighbor_cell_index = cell.neighbors[sun_direction as usize];

                if neighbor_cell_index == -1 {
                    // no cell in that direction
                    return false;
                }

                let neighbor_tree = game_state.trees
                    .iter()
                    .find(|tree| tree.cell_index == neighbor_cell_index)
                    .unwrap_or(
                        &(Tree {
                            cell_index: -1,
                            size: 0,
                            is_mine: false,
                            is_dormant: false,
                            is_shadowed: false,
                        })
                    );

                if neighbor_tree.cell_index != -1 && neighbor_tree.size >= tree_size {
                    return true;
                }

                if count > 1 {
                    return check_neighbor(game_state, neighbor_cell_index, count - 1, tree_size);
                } else {
                    return false;
                }
            }

            check_neighbor(game_state, cell_index, count, tree_size)
        }
        None => {
            return false;
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum ActionType {
    Wait,
    Seed,
    Grow,
    Complete,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Action {
    action_type: ActionType,
    source_cell_index: Option<i8>,
    target_cell_index: Option<i8>,
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
        ActionType::Seed => Some(parts[1].parse::<i8>().unwrap_or(-1)),
        _ => None,
    };

    let target_cell_index = match action_type {
        ActionType::Seed => Some(parts[2].parse::<i8>().unwrap_or(-1)),
        ActionType::Wait => None,
        _ => Some(parts[1].parse::<i8>().unwrap_or(-1)),
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
                action.source_cell_index.unwrap_or(-1),
                action.target_cell_index.unwrap_or(-1)
            ),
        ActionType::Grow => format!("GROW {}", action.target_cell_index.unwrap_or(-1)),
        ActionType::Complete => format!("COMPLETE {}", action.target_cell_index.unwrap_or(-1)),
    }
}

// =================================================================================================
// =================================================================================================

fn calculate_sun_points(game_state: &GameState) -> (u8, u8) {
    // The forest's lesser spirits will harvest sun points from each tree that is not hit by a spooky shadow.
    // The points will be given to the owner of the tree.
    // The number of sun points harvested depends on the size of the tree:

    //     A size 0 tree (a seed): no points.
    //     A size 1 tree: 1 sun point.
    //     A size 2 tree: 2 sun points.
    //     A size 3 tree: 3 sun points.

    let mut my_sun = game_state.my_sun;
    let mut opp_sun = game_state.opp_sun;

    for tree in game_state.trees.iter() {
        if !tree.is_shadowed {
            match tree.size {
                1 => {
                    if tree.is_mine {
                        my_sun += 1;
                    } else {
                        opp_sun += 1;
                    }
                }
                2 => {
                    if tree.is_mine {
                        my_sun += 2;
                    } else {
                        opp_sun += 2;
                    }
                }
                3 => {
                    if tree.is_mine {
                        my_sun += 3;
                    } else {
                        opp_sun += 3;
                    }
                }
                _ => {}
            }
        }
    }

    (my_sun, opp_sun)
}

// ------------------------------------------------------------------
// Action Logic
// ------------------------------------------------------------------

fn choose_action(game_state: &GameState) -> Action {
    run_mcts(game_state, 1000)
}

fn apply_action(game_state: &mut GameState, action: &Action) -> GameState {
    let mut new_game_state = game_state.clone();

    // TODO: need to finish finding shadows before able to calculate sun points to apply action to game state

    match action.action_type {
        ActionType::Wait => {
            let (my_sun, opp_sun) = calculate_sun_points(&new_game_state);
            new_game_state.my_sun = my_sun;
            new_game_state.opp_sun = opp_sun;
        }
        _ => {}
        // ActionType::Seed => {
        //     let source_tree = new_game_state.trees
        //         .iter_mut()
        //         .find(|tree| tree.cell_index == action.source_cell_index.unwrap())
        //         .unwrap();

        //     source_tree.is_dormant = true;

        //     let target_cell = new_game_state.cells
        //         .iter()
        //         .find(|cell| cell.index == action.target_cell_index.unwrap())
        //         .unwrap();

        //     new_game_state.my_sun -= 4;

        //     new_game_state.trees.push(Tree {
        //         cell_index: target_cell.index,
        //         size: 0,
        //         is_mine: true,
        //         is_dormant: true,
        //         is_shadowed: is_shadowed(&new_game_state, target_cell.index),
        //     });
        // }
        // ActionType::Grow => {
        //     let target_tree = new_game_state.trees
        //         .iter_mut()
        //         .find(|tree| tree.cell_index == action.target_cell_index.unwrap())
        //         .unwrap();

        //     target_tree.is_dormant = true;
        //     target_tree.size += 1;

        //     new_game_state.my_sun -= match target_tree.size {
        //         1 => 1,
        //         2 => 3,
        //         3 => 7,
        //         _ => panic!("Invalid tree size"),
        //     };
        // }
        // ActionType::Complete => {
        //     let target_tree = new_game_state.trees
        //         .iter_mut()
        //         .find(|tree| tree.cell_index == action.target_cell_index.unwrap())
        //         .unwrap();

        //     target_tree.is_dormant = true;

        //     new_game_state.my_sun -= 4;

        //     new_game_state.my_score += match target_tree.size {
        //         1 => 1,
        //         2 => 3,
        //         3 => 7,
        //         _ => panic!("Invalid tree size"),
        //     };
        // }
    }

    new_game_state
}

// ------------------------------------------------------------------
// MCTS
// ------------------------------------------------------------------

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

#[derive(Clone)]
struct Node {
    parent: Option<Box<Node>>,
    children: Vec<Node>,
    visit_count: u32,
    score: f32,
    game_state: GameState,
    action: Option<Action>,
}

fn get_ucb1_score(node: &Node) -> f32 {
    if node.visit_count == 0 {
        return f32::MAX;
    }

    let parent_visit_count = node.parent.as_ref().unwrap_or(
        &Box::new(Node {
            parent: None,
            children: Vec::new(),
            visit_count: 0,
            score: 0.0,
            game_state: GameState::new(),
            action: None,
        })
    ).visit_count as f32;
    let node_visit_count = node.visit_count as f32;
    let node_score = node.score as f32;

    node_score + 2.0 * (parent_visit_count.ln() / node_visit_count).sqrt()
}

fn get_best_node(node: Node) -> Node {
    if node.children.len() == 0 {
        return node;
    }

    let mut best_node = node.children
        .first()
        .unwrap_or(
            &(Node {
                parent: None,
                children: Vec::new(),
                visit_count: 0,
                score: 0.0,
                game_state: GameState::new(),
                action: None,
            })
        )
        .clone();

    for child in node.children.iter() {
        if get_ucb1_score(child) > get_ucb1_score(&best_node) {
            best_node = child.clone();
        }
    }

    best_node.clone()
}

struct MCTS {
    root_node: Node,
}

fn run_mcts(game_state: &GameState, n: u32) -> Action {
    let mcts = MCTS {
        root_node: Node {
            parent: None,
            children: Vec::new(),
            visit_count: 0,
            score: 0.0,
            game_state: game_state.clone(),
            action: None,
        },
    };

    for _ in 0..n {
        let mut node = mcts.root_node.clone();

        while node.children.len() > 0 {
            node = get_best_node(node);
        }

        let mut new_node = Node {
            parent: Some(Box::new(node.clone())),
            children: Vec::new(),
            visit_count: 0,
            score: 0.0,
            game_state: node.game_state.clone(),
            action: None,
        };

        let possible_actions = new_node.game_state.possible_actions.clone();

        for action in possible_actions.iter() {
            new_node.action = Some(action.clone());
            new_node.game_state = apply_action(&mut new_node.game_state, action);
            new_node.score = evaluate_state(&new_node.game_state);
            node.children.push(new_node.clone());
        }
    }

    // Find the child node of the root with the highest score
    let best_node = mcts.root_node.children
        .iter()
        .max_by(|x, y| x.score.partial_cmp(&y.score).unwrap())
        .unwrap();

    best_node.action.clone().unwrap()
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
            index: index as i8,
            richness: richness as u8,
            neighbors: vec![
                neigh_0 as i8,
                neigh_1 as i8,
                neigh_2 as i8,
                neigh_3 as i8,
                neigh_4 as i8,
                neigh_5 as i8
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
            let cell_index = parse_input!(inputs[0], i8); // location of this tree
            let size = parse_input!(inputs[1], u8); // size of this tree: 0-3
            let is_mine = parse_input!(inputs[2], u8); // 1 if this is your tree
            let is_dormant = parse_input!(inputs[3], u8); // 1 if this tree is dormant
            let is_shadowed = is_shadowed(&game_state, cell_index);

            trees.push(Tree {
                cell_index,
                size,
                is_mine: is_mine == 1,
                is_dormant: is_dormant == 1,
                is_shadowed,
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
