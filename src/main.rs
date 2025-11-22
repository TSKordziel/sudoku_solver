use petgraph::graph::UnGraph;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
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
    possible_values: HashSet<Color>
}
impl Cell {
    fn new() -> Cell{
        let mut  colors_set: HashSet<Color> = HashSet::new();
        for color in COLORS{
            colors_set.insert(color);
        }
        Cell { value: None, possible_values: colors_set }
    }
}
struct SudokuBoard {
    grid: [[Cell; 9]; 9],
    constraints: UnGraph<(usize, usize), ()>
}

impl SudokuBoard {
    fn new() -> SudokuBoard{
        let mut board: SudokuBoard = SudokuBoard { grid: [[Cell::new(); 9]; 9], constraints: UnGraph::new_undirected() };

        for (i,row) in board.grid.iter().enumerate(){
            for (j, colum) in row.iter().enumerate(){
                board.constraints.add_node((i,j));
            }
        }
        
    }
    
}





fn main() {
    println!("Hello, world!");
}
