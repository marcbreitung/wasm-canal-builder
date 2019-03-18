#![allow(dead_code)]

use crate::block::Block;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    row: u32,
    col: u32,
}

impl State {
    pub fn new(row: u32, col: u32) -> Self {
        State { row, col }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Root {
        state: State,
    },
    Child {
        state: State,
        parent: State,
    },
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        use self::Node::*;
        match (self, other) {
            (Root { state: a }, Child { state: b, parent: _ }) => a == b,
            (Child { state: a, parent: _ }, Root { state: b }) => a == b,
            (Child { state: a, parent: _ }, Child { state: b, parent: _ }) => a == b,
            _ => false,
        }
    }
}

pub struct Search {
    width: u32,
    height: u32,
    start: State,
    goal: State,
    graph: Vec<Block>,
    frontier: Vec<Node>,
    explored: Vec<Node>,
}

impl Search {
    pub fn new(graph: Vec<Block>, start: State, goal: State, width: u32, height: u32) -> Self {
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

    pub fn search(&mut self) -> Vec<u32> {
        let solution = self.search_solution();
        let solution_list = self.get_solution(solution);
        solution_list
    }

    fn get_solution(&mut self, solution: Option<Node>) -> Vec<u32> {
        let mut solution_list: Vec<u32> = vec![];

        match solution {
            Some(node) => {
                let mut looping = true;
                let mut x = node;
                while looping {
                    match x {
                        Node::Root { state: _ } => {
                            looping = false;
                        }
                        Node::Child { state, parent } => {
                            let index = self.get_index_by_state(parent.clone());
                            x = self.explored.remove(index);
                            solution_list.push(state.col);
                            solution_list.push(state.row);
                        }
                    };
                }
            }
            None => {}
        }

        solution_list.iter().rev().cloned().collect::<Vec<u32>>()
    }

    fn search_solution(&mut self) -> Option<Node> {
        let mut solution = None;

        if self.start == self.goal {
            return Some(Node::Root { state: self.start.clone() });
        }

        self.frontier.push(Node::Root { state: self.start.clone() });

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

        let parent: State = match node {
            Node::Root { state } => state,
            Node::Child { state, parent: _ } => state,
        };

        for action in actions.iter().cloned() {
            let extended_node = Node::Child { state: action.clone(), parent: parent.clone() };

            let in_explored = self.explored.iter().any(|x| *x == extended_node);
            let in_frontier = self.frontier.iter().any(|x| *x == extended_node);

            if in_explored == false && in_frontier == false {
                if action == self.goal {
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

        let state = match node {
            Node::Root { state } => state,
            Node::Child { state, parent: _ } => state,
        };

        let rows = vec![state.row as i32 - 1, state.row as i32, state.row as i32 + 1, state.row as i32];
        let cols = vec![state.col as i32, state.col as i32 + 1, state.col as i32, state.col as i32 - 1];

        let mut iter = rows.iter().zip(cols.iter());

        loop {
            match iter.next() {
                Some((row, col)) if *row >= 0 && *col >= 0 => {
                    let index = self.get_index(*row as u32, *col as u32);
                    if index < self.graph.len() {
                        match self.graph[index] {
                            Block::Path => {
                                let action = State::new(*row as u32, *col as u32);
                                actions.push(action);
                            }
                            _ => {}
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

    fn get_index_by_state(&self, s: State) -> usize {
        let mut index = 0;

        for (i, node) in self.explored.iter().enumerate() {
            match node {
                Node::Root { state } if *state == s => index = i,
                Node::Child { state, parent: _ } if *state == s => index = i,
                _ => {}
            }
        }

        index
    }
}