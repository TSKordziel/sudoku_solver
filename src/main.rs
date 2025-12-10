use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use std::collections::hash_set;
use std::collections::HashSet;
use std::usize;

const TEST_PUZZLE: [[&str; 9]; 9] = [
    ["5", "3", ".", ".", "7", ".", ".", ".", "."],
    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    [".", "9", "8", ".", ".", ".", ".", "6", "."],
    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    [".", "6", ".", ".", ".", ".", "2", "8", "."],
    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    [".", ".", ".", ".", "8", ".", ".", "7", "9"],
];

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
    buckets: [HashSet<(usize, usize)>; 10],
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        let mut constraints: UnGraph<(usize, usize), ()> = UnGraph::new_undirected();
        let grid: [[Cell; 9]; 9] = std::array::from_fn(|row| {
            std::array::from_fn(|col| Cell::new(constraints.add_node((row, col))))
        });
        let mut buckets: [HashSet<(usize, usize)>; 10] = std::array::from_fn(|_| HashSet::new());
        for (row, row_data) in grid.iter().enumerate() {
            for (col, cell) in row_data.iter().enumerate() {
                let num_cell_possible_values = cell.possible_values.len();
                buckets[num_cell_possible_values].insert((row, col));
            }
        }
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
        for block_row in 0..3 {
            for block_col in 0..3 {
                let mut nodes: Vec<NodeIndex> = Vec::new();
                for row in (block_row * 3)..(block_row * 3 + 3) {
                    for col in (block_col * 3)..(block_col * 3 + 3) {
                        nodes.push(grid[row][col].graph_index);
                    }
                }
                for index_a in 0..9 {
                    for index_b in (index_a + 1)..9 {
                        let node_index1 = nodes[index_a];
                        let node_index2 = nodes[index_b];
                        if constraints.find_edge(node_index1, node_index2).is_none() {
                            constraints.add_edge(node_index1, node_index2, ());
                        }
                    }
                }
            }
        }
        SudokuBoard {
            grid,
            constraints,
            buckets,
        }
    }
    fn get_cell_constraint_neighbors(&self, grid_index: (usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = grid_index;
        self.constraints
            .neighbors(self.grid[row][col].graph_index)
            .map(|neighbor| {
                if let Some(&index) = self.constraints.node_weight(neighbor) {
                    index
                } else {
                    (row, col)
                }
            })
            .collect()
    }
    fn brodcast_cell_color_to_neighbors(&mut self, grid_index: (usize, usize)) {
        let (cell_row, cell_col) = grid_index;
        if let Some(color) = self.grid[cell_row][cell_col].value {
            for (row, col) in self.get_cell_constraint_neighbors((cell_row, cell_col)) {
                let old_count = self.grid[row][col].possible_values.len();
                self.grid[row][col].possible_values.remove(&color);
                let new_count = self.grid[row][col].possible_values.len();
                self.buckets[old_count].remove(&(row, col));
                self.buckets[new_count].insert((row, col));
            }
        }
    }
    fn set_cell_color(&mut self, grid_index: (usize, usize), color: Color) {
        let (cell_row, cell_col) = grid_index;
        self.grid[cell_row][cell_col].value = Some(color);
        let old_count = self.grid[cell_row][cell_col].possible_values.len();
        self.grid[cell_row][cell_col].possible_values.clear();
        let new_count: usize = self.grid[cell_row][cell_col].possible_values.len();
        self.buckets[old_count].remove(&(cell_row, cell_col));
        self.buckets[new_count].insert((cell_row, cell_col));
        self.brodcast_cell_color_to_neighbors((cell_row, cell_col));
    }
    fn initialize_board_colors(&mut self, puzzle: [[&str; 9]; 9]) {
        for (row, row_data) in puzzle.iter().enumerate() {
            for (col, &puzzle_cell) in row_data.iter().enumerate() {
                match puzzle_cell {
                    "1" => {
                        self.set_cell_color((row, col), Color::One);
                    }
                    "2" => {
                        self.set_cell_color((row, col), Color::Two);
                    }
                    "3" => {
                        self.set_cell_color((row, col), Color::Three);
                    }
                    "4" => {
                        self.set_cell_color((row, col), Color::Four);
                    }
                    "5" => {
                        self.set_cell_color((row, col), Color::Five);
                    }
                    "6" => {
                        self.set_cell_color((row, col), Color::Six);
                    }
                    "7" => {
                        self.set_cell_color((row, col), Color::Seven);
                    }
                    "8" => {
                        self.set_cell_color((row, col), Color::Eight);
                    }
                    "9" => {
                        self.set_cell_color((row, col), Color::Nine);
                    }
                    _ => {}
                }
            }
        }
    }
    fn set_hanging_singles(&mut self) {
        loop {
            let singles: Vec<_> = self.buckets[1].iter().copied().collect();
            if singles.is_empty() {
                break;
            }
            for cell in singles {
                let (row, col) = cell;
                if let Some(&color) = self.grid[row][col].possible_values.iter().next() {
                    self.set_cell_color(cell, color);
                }
            }
        }
    }
}

fn main() {
    let mut board = SudokuBoard::new();
    let counts_1: [usize; 10] = std::array::from_fn(|index| board.buckets[index].len());

    println!("{:?}", counts_1);

    board.initialize_board_colors(TEST_PUZZLE);
    board.set_hanging_singles();
    let counts_1: [usize; 10] = std::array::from_fn(|index| board.buckets[index].len());

    println!("{:?}", counts_1);
}
