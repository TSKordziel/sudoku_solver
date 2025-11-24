use petgraph::graph::{GraphIndex, UnGraph};
use petgraph::graph::NodeIndex;
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
    Nine
}

const COLORS: [Color; 9] = [Color::One, Color::Two, Color::Three, Color::Four, Color::Five, Color::Six, Color::Seven, Color::Eight, Color::Nine];

struct Cell {
    value: Option<Color>,
    possible_values: HashSet<Color>,
    graph_index: NodeIndex,
}
impl Cell {
    fn new(node_index: NodeIndex) -> Cell{
        let mut  colors_set: HashSet<Color> = HashSet::new();
        for color in COLORS{
            colors_set.insert(color);
        }
        Cell { value: None, possible_values: colors_set, graph_index: node_index}
    }
}
struct SudokuBoard {
    grid: [[Cell; 9]; 9],
    constraints: UnGraph<(usize, usize), ()>
}

impl SudokuBoard {
    fn new() -> SudokuBoard{
        let mut constraints: UnGraph<(usize, usize), ()> = UnGraph::new_undirected();
        let mut grid:[[Cell; 9]; 9] = std::array::from_fn(|row| {
            std::array::from_fn(|col| {
                Cell::new(constraints.add_node((row,col))) ;
            })
        });
        todo!();
        SudokuBoard { grid, constraints }
    }
    
}





fn main() {
    println!("Hello, world!");
}
