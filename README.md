# Sudoku Solver (Rust + Graph Theory)

A fully featured Sudoku solver implemented in Rust using **graph theory**, **constraint propagation**, and an optimized **backtracking search**.  
Every Sudoku cell is modeled as a node in a Petgraph `UnGraph`, and constraint relationships (row, column, box) are encoded as edges.  
The solver aggressively prunes the search space using “hanging singles,” candidate buckets, and an MRV heuristic.

---

## ✨ Features

- **Graph-Modeled Sudoku Board**
  - Each cell is a graph node; edges model all constraint relationships.
  - Neighbor lookups are O(1) and cleanly abstracted.

- **Constraint Propagation Engine**
  - Maintains a bucketed index of cells grouped by number of possible values (1–9).
  - Automatically resolves “hanging singles” before any backtracking begins.
  - Propagates value assignments to all dependent neighbors.

- **Optimized Backtracking Solver**
  - Uses **MRV (Minimum Remaining Values)** to pick the next cell.
  - Clones board state for safe, isolated recursive exploration.
  - Prunes invalid branches early via contradiction checks.

- **Reliable Solving**
  - Successfully solves medium → expert puzzles.
  - Deterministic, efficient, and built using idiomatic Rust patterns.

---
