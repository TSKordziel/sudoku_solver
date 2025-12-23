# Sudoku Solver (Rust + Graph Theory)

A fully featured Sudoku solver implemented in Rust using **graph theory**, **constraint propagation**, and an optimized **backtracking search**.  
Every Sudoku cell is modeled as a node in a Petgraph `UnGraph`, and constraint relationships (row, column, box) are encoded as edges.  
The solver aggressively prunes the search space using “hanging singles,” candidate buckets, and an MRV heuristic.

---

## 🎯 Project Purpose

This project was built with two deliberate learning objectives:

1. **Develop a deep, practical understanding of graph data structures**  
   The Sudoku board is modeled explicitly as a graph, forcing all relationships—rows, columns, and subgrids—to be expressed through edges and neighbor traversal rather than ad-hoc indexing. This approach emphasizes thinking in terms of connectivity, adjacency, and propagation, not just arrays.

2. **Implement a depth-first search algorithm via backtracking on graph-based state**  
   The solver applies a recursive DFS strategy (backtracking) over the graph, exploring candidate assignments while safely isolating state through cloning. Constraint propagation acts as an early-exit mechanism, dramatically reducing the DFS search space.

The goal was not merely to “solve Sudoku,” but to use a familiar problem to internalize graph-driven modeling and backtracking search patterns that generalize to far more complex constraint-satisfaction problems.

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
