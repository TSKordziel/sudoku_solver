use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use std::collections::hash_set;
use std::collections::HashSet;
use std::usize;

/* const TEST_PUZZLE: [[Option<Color>; 9]; 9] = [
    [
        None,
        Some(Color::Six),
        None,
        Some(Color::Three),
        Some(Color::Seven),
        Some(Color::Eight),
        None,
        None,
        Some(Color::Five),
    ],
    [
        None,
        None,
        Some(Color::Four),
        None,
        None,
        Some(Color::One),
        Some(Color::Nine),
        Some(Color::Two),
        None,
    ],
    [],
    [],
    [],
    [],
    [],
    [],
    [],
]; */

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
        let new_count: usize = 1;
        self.buckets[old_count].remove(&(cell_row, cell_col));
        self.buckets[new_count].insert((cell_row, cell_col));
        self.brodcast_cell_color_to_neighbors((cell_row, cell_col));
    }
    fn initialize_board_colors(&mut self, puzzle: [[Option<Color>; 9]; 9]) {
        for (row, row_data) in puzzle.iter().enumerate() {
            for (col, &puzzle_cell) in row_data.iter().enumerate() {
                if let Some(color) = puzzle_cell {
                    self.set_cell_color((row, col), color);
                }
            }
        }
    }
}

fn main() {
    let mut board = SudokuBoard::new();

    board.set_cell_color((0, 0), Color::One);
    board.set_cell_color((0, 8), Color::Two);

    let neighbors = board.get_cell_constraint_neighbors((0, 0));

    println!("{:?}", neighbors);
    println!("{:?}, ", board.buckets[9].len());
}
