#[derive(Debug, Copy, Clone)]
struct State {
    row: u32,
    col: u32,
}

impl State {
    fn new(row: u32, col: u32) -> Self {
        State { row, col }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.row == other.row && self.col == other.col
    }
}

#[derive(Debug, Clone)]
struct Node {
    state: State,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(state: State, parent: Option<Box<Node>>) -> Self {
        Node { state, parent }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.state.row == other.state.row && self.state.col == other.state.col
    }
}

struct Search {
    width: u32,
    height: u32,
    start: State,
    goal: State,
    graph: Vec<u8>,
    frontier: Vec<Node>,
    explored: Vec<Node>,
}

impl Search {
    fn new(graph: Vec<u8>, start: State, goal: State, width: u32, height: u32) -> Self {
        Search {
            width,
            height,
            start,
            goal,
            graph,
            frontier: vec![],
            explored: vec![],
        }
    }

    fn search(&mut self) -> Option<Node> {
        let mut solution = None;

        if self.start == self.goal {
            return Some(Node::new(self.start, None));
        }

        self.frontier.push(Node::new(self.start, None));

        while self.frontier.len() > 0 {
            let node = self.frontier.remove(0);
            self.explored.push(node.clone());

            match self.extend(node) {
                Some(solution_node) => {
                    self.frontier = vec![];
                    solution = Some(solution_node);
                }
                None => ()
            }
        }

        solution
    }

    fn extend(&mut self, node: Node) -> Option<Node> {
        let mut solution = None;

        let actions = self.get_actions(&node);

        for action in actions.iter().cloned() {
            let extended_node = Node::new(action, Some(Box::new(node.clone())));

            let in_explored = self.explored.iter().any(|x| *x == extended_node);
            let in_frontier = self.frontier.iter().any(|x| *x == extended_node);

            if in_explored == false && in_frontier == false {
                if extended_node.state == self.goal {
                    solution = Some(extended_node);
                } else {
                    self.frontier.push(extended_node);
                }
            }
        };

        solution
    }

    fn get_actions(&self, node: &Node) -> Vec<State> {
        let mut actions = vec![];

        let cols = vec![node.state.col as i32, node.state.col as i32 + 1, node.state.col as i32, node.state.col as i32 - 1];
        let rows = vec![node.state.row as i32 - 1, node.state.row as i32, node.state.row as i32 + 1, node.state.row as i32];

        let mut iter = cols.iter().zip(rows.iter());

        loop {
            match iter.next() {
                Some((row, col)) if *row >= 0 && *col >= 0 => {
                    let index = self.get_index(*row as u32, *col as u32);
                    if index < self.graph.len() {
                        if self.graph[index] as u8 == 1 {
                            let action = State::new(*row as u32, *col as u32);
                            actions.push(action);
                        }
                    }
                }
                Some(_) => {}
                None => { break; }
            }
        }

        actions
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}
