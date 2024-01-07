use std::io;
use std::fmt;
use rand::seq::SliceRandom;

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

#[derive(Clone, Copy)]
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

fn get_is_shadowed(state: &GameState, size: i32, cell_index: i32) -> bool {
    fn check_neighbor(
        state: &GameState,
        root_tree_size: i32,
        sun_index: i32,
        cell_index: i32,
        count: i32
    ) -> bool {
        // Get reference to cell
        let cell = state.area
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
        let neighbor_tree = state.forest.iter().find(|tree| tree.cell_index == neighbor_index);

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
        check_neighbor(state, root_tree_size, sun_index, neighbor_index, count - 1)
    }

    // Index of sun direction
    let sun_index = ((state.day % 6) + 3) % 6;

    // How many neighbors to check
    let count = size;

    // Start recursive check
    check_neighbor(state, size, sun_index, cell_index, count)
}

// ================================================================================================
// Action
// ================================================================================================

#[derive(Clone, Copy)]
#[derive(PartialEq)]
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
// GameState
// ================================================================================================

#[derive(Clone)]
struct GameState {
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

fn get_game_state(area: Area) -> GameState {
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

    GameState {
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

fn get_new_state(state: GameState, action: Action) -> GameState {
    let mut new_state = state.clone();

    match action {
        Action::Grow(cell_index) => {}
        Action::Seed(source_index, target_index) => {}
        Action::Complete(cell_index) => {}
        Action::Wait => {}
    }

    new_state.day += 1;
    new_state.op_is_waiting = false;

    new_state
}

// ================================================================================================
// MCTS
// ================================================================================================

#[derive(Clone)]
// TODO: I should change parent and children to indexes instead,
// because storing nodes (that also store nodes, that also store nodes, etc) will be a lot of memory (and slow)
// it may also help my mutablility issues
struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    children: Vec<Node<'a>>,
    action: Action,
    visits: i32,
    wins: i32,
    state: GameState,
}

fn mcts(root_node: &Node, iterations: i32) -> Action {
    for _ in 0..iterations {
        let leaf_node = traverse(root_node);
        leaf_node.visits += 1;
        let is_win = rollout(leaf_node);

        if is_win {
            leaf_node.wins += 1;
        }

        backpropagate(leaf_node, is_win);
    }

    return best_action(root_node);
}

fn traverse<'a>(root_node: &'a mut Node<'a>) -> &'a mut Node<'a> {
    let mut current_node = root_node;

    while is_fully_expanded(current_node) {
        current_node = select_child_by_utc(current_node);
    }

    // If the game day is  23, then the game is over
    if current_node.state.day == 23 {
        return current_node;
    }

    return expand_and_select_child(current_node);
}

fn is_fully_expanded(node: &Node) -> bool {
    node.children.len() == node.state.action_list.len()
}

fn expand_and_select_child<'a>(parent_node: &'a mut Node) -> &'a mut Node<'a> {
    let unused_actions: Vec<Action> = parent_node.state.action_list
        .iter()
        .filter(|action| { !parent_node.children.iter().any(|child| child.action == **action) })
        .cloned()
        .collect();

    let random_action: &Action = unused_actions.choose(&mut rand::thread_rng()).unwrap();
    let mut new_state = get_new_state(parent_node.state, *random_action);

    let new_node = Node {
        parent: Some(parent_node),
        children: vec![],
        action: *random_action,
        visits: 0,
        wins: 0,
        state: new_state,
    };

    parent_node.children.push(new_node);
    return parent_node.children.last_mut().unwrap();
}

fn rollout(node: &Node) -> bool {
    let mut current_node = node.clone();
    while current_node.state.day <= 23 {
        current_node = rollout_policy(current_node);
    }
    return current_node.state.score > current_node.state.op_score;
}

fn rollout_policy(node: Node) -> Node {
    // From this node, choose a random action, and get the new state, and return the new node (with the new state)
    let random_action: &Action = node.state.action_list.choose(&mut rand::thread_rng()).unwrap();
    let mut new_state = get_new_state(node.state.clone(), *random_action);

    let new_node = Node {
        parent: None,
        children: vec![],
        action: *random_action,
        visits: 0,
        wins: 0,
        state: new_state,
    };

    return new_node;
}

fn backpropagate(node: &mut Node, is_win: bool) {
    if node.parent.is_none() {
        // This is the root node
        return;
    }

    let node_parent = node.parent.as_mut().unwrap();
    node_parent.visits += 1;

    if is_win {
        node_parent.wins += 1;
    }

    backpropagate(node_parent, is_win)
}

fn uct_value(current_node: &Node, parent_node: &Node, explore_rate: f32) -> f32 {
    if current_node.visits == 0 {
        if explore_rate == 0.0 {
            return f32::INFINITY;
        } else {
            return 0.0;
        }
    }
    let wins = current_node.wins as f32;
    let visits = current_node.visits as f32;
    let parent_visits = parent_node.visits as f32;
    wins / visits + explore_rate * (parent_visits.ln() / visits).sqrt()
}

fn select_child_by_utc<'a>(node: &'a Node<'a>) -> &'a Node<'a> {
    let mut best_utc = f32::NEG_INFINITY;
    let mut best_child_index = 0;

    for (i, child) in node.children.iter().enumerate() {
        let utc = uct_value(child, node, 1.0);
        if utc > best_utc {
            best_utc = utc;
            best_child_index = i;
        }
    }

    // Get a mutable reference to the best child
    return node.children.get(best_child_index).unwrap();
}

fn best_action(node: &Node) -> Action {
    // Find the child with the most visits
    node.children
        .iter()
        .max_by_key(|child| child.visits)
        .unwrap().action
}

/*

Things we will need for simulation:

- Determine the game state based on actions taken by both players for the given turn
  - This includes the ability to get the following:
    - Available actions for each player
    - Each players sun points
    - Each players score
    - Each players forest
    - Nutrients
    - Day


*/

// ================================================================================================
// Main
// ================================================================================================

fn main() {
    let area = get_area();

    // game loop
    loop {
        let answer = String::from("WAIT");

        let state = get_game_state(area);

        for a in state.action_list.iter() {
            eprintln!("Action : {}", a);
        }

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("{}", answer);
    }
}
