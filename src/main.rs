use petgraph::graph::NodeIndex;
use petgraph::graph::{GraphIndex, UnGraph};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

const COLORS: [Color; 9] = [
    Color::One,
    Color::Two,
    Color::Three,
    Color::Four,
    Color::Five,
    Color::Six,
    Color::Seven,
    Color::Eight,
    Color::Nine,
];

struct Cell {
    value: Option<Color>,
    possible_values: HashSet<Color>,
    graph_index: NodeIndex,
}
impl Cell {
    fn new(node_index: NodeIndex) -> Cell {
        let mut colors_set: HashSet<Color> = HashSet::new();
        for color in COLORS {
            colors_set.insert(color);
        }
        Cell {
            value: None,
            possible_values: colors_set,
            graph_index: node_index,
        }
    }
}
struct SudokuBoard {
    grid: [[Cell; 9]; 9],
    constraints: UnGraph<(usize, usize), ()>,
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        let mut constraints: UnGraph<(usize, usize), ()> = UnGraph::new_undirected();
        let mut grid: [[Cell; 9]; 9] = std::array::from_fn(|row| {
            std::array::from_fn(|col| Cell::new(constraints.add_node((row, col))))
        });
        for row in 0..9 {
            for c1 in 0..9 {
                for c2 in (c1 + 1)..9 {
                    let node_index1 = grid[row][c1].graph_index;
                    let node_index2 = grid[row][c2].graph_index;
                    constraints.add_edge(node_index1, node_index2, ());
                }
            }
        }
        for col in 0..9 {
            for r1 in 0..9 {
                for r2 in (r1 + 1)..9 {
                    let node_index1 = grid[r1][col].graph_index;
                    let node_index2 = grid[r2][col].graph_index;
                    constraints.add_edge(node_index1, node_index2, ());
                }
            }
        }

        SudokuBoard { grid, constraints }
    }
}

fn main() {
    println!("Hello, world!");
}
