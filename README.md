# Competitive Programming Library (Rust)

A comprehensive Rust library of algorithms, data structures, and problem-solving frameworks for competitive programming (AtCoder, Codeforces, etc.). All templates and implementations are optimized with time complexity $O(\cdot)$ and space complexity in mind.

---

## 📁 Directory Structure

```text
library/
├── algo/                    # Fundamental algorithms & implementation techniques
├── all_search/              # Exhaustive search patterns (Bit, Permutation, Recursion, Sqrt)
├── binary/                  # Binary search (Binary search on answer, Boundary identification)
├── dp/                      # Dynamic Programming (Digit DP, Bit DP, Tree DP, Interval DP, Re-rooting DP, etc.)
├── flow/                    # Network flow & matching (Dinic, Ford-Fulkerson, Bipartite matching)
├── general/                 # General competitive programming notes, checklists & Rust tips
│   ├── compute.md           # Time complexity estimation guide
│   ├── cornercase_fix.md    # Checklist for edge cases & boundary conditions
│   ├── cp-tricks.md         # General competitive programming tips & tricks
│   ├── problem_solving.md   # Step-by-step problem solving methodology
│   ├── solution_strategy.md # Comprehensive problem-solving techniques & strategies
│   └── tricks.rs            # Rust language-specific implementation tricks
├── graph/                   # Graph & tree algorithms (Dijkstra, BFS/DFS, MST, SCC, LCA, etc.)
├── greedy/                  # Greedy algorithms (Priority queue, Sweepline, Lexicographical optimization)
├── heuristic/               # Heuristic optimization (Simulated Annealing, Beam Search, Hill Climbing)
├── idea/                    # Problem-solving frameworks & paradigms (Contribution sum, Reverse queries, Parity)
├── io/                      # Input/Output optimization (Fast I/O, proconio templates)
├── numbers/                 # Mathematics, number theory, combinatorics & geometry (ModInt, Primes, Matrix, Bit)
├── semi_all/                # Semi-exhaustive search (Pruned search, Meet-in-the-middle)
├── sort/                    # Custom sorting (Angle sort, Key sort)
├── span/                    # Array span & sliding window operations (Two Pointers, Imos 1D/2D, RLE)
├── strings/                 # String algorithms (Rolling Hash, KMP, Z-algorithm, Palindrome detection)
├── structures/              # Data structures (SegTree, LazySegTree, DSU, Fenwick/BIT, SparseTable)
└── top_coder/               # Generic templates & utility functions
```

---

## 📚 Category Breakdown

### 1. `algo/` - Fundamental Algorithms & Implementation Techniques

General-purpose algorithm implementation patterns.

- **`compress/`**: Coordinate Compression
- **`grid/`**: Grid graph navigation & direction vectors
- **`impl/`**: Miscellaneous implementation techniques
- **`inversion/`**: Inversion number calculation
- **`matrix/`**: Matrix operations & transformations
- **`pointer/`**: Pointer & index scanning
- **`recursive/`**: Recursion patterns
- **`scanline/`**: Scanline algorithm
- **`segment/`**: Segment partitioning & manipulations
- **`square_root_decomposition/`**: Square Root (Sqrt) Decomposition

### 2. `all_search/` - Exhaustive Search

Patterns for searching the entire solution space.

- **`bit/`**: Bitwise brute force ($2^N$ search)
- **`enum/`**: State & combination enumeration
- **`impl/`**: Implementation patterns for exhaustive search
- **`permutation/`**: Permutation search ($N!$ search)
- **`sqrt/`**: Sqrt-based exhaustive search

### 3. `binary/` - Binary Search

Broad binary search paradigms and boundary detection.

- **`ans_binary/`**: Binary search on answer
- **`less_than_identify/`**: Identifying boundary indices satisfying specific conditions

### 4. `dp/` - Dynamic Programming

Comprehensive library of DP techniques commonly appearing in competitive programming.

- **`bidirectional_dp/`**: Bidirectional DP
- **`binarytree_dp/`**: Binary Tree DP
- **`bit_sales_matching/`**: Bit DP, Traveling Salesperson Problem (TSP), and matching DP
- **`corner_case_initialization/`**: DP initialization & boundary setup
- **`digit/`**: Digit DP
- **`doubling/`**: Doubling DP
- **`lis/`**: Longest Increasing Subsequence (LIS)
- **`memo/`**: Memoized recursion
- **`nega_min_max/`**: Game DP (Negamax / Min-Max)
- **`prefix_suffix_dp/`**: Prefix/Suffix DP
- **`re-rooting/`**: Re-rooting DP (All-tree DP)
- **`reconstruct/`**: DP path reconstruction
- **`segdp/`**: Segment Tree accelerated DP
- **`spandp/`**: Interval/Span DP
- **`topological_dp/`**: Topological sort order DP
- **`tree_graph_dp/`**: Tree / Graph DP

### 5. `flow/` - Network Flow & Matching

Maximum flow, minimum cut, and matching algorithms.

- **`bipartite_matching/`**: Bipartite Matching
- **`dinic/`**: Dinic's algorithm ($O(V^2 E)$)
- **`fordfulkerson/`**: Ford-Fulkerson algorithm
- **`edmonskarp/`**: Edmonds-Karp algorithm

### 6. `general/` - General Documentation & Notes

Collection of competitive programming documentation, checklists, and language-specific utilities.

- **[compute.md]**: Time complexity estimation guide ($N$ vs. acceptable Big-O complexity).
- **[cornercase_fix.md]**: Edge cases & boundary condition checklist.
- **[cp-tricks.md]**: Competitive programming tips, constants, and Rust debugging techniques.
- **[problem_solving.md]**: Step-by-step problem solving approach checklist.
- **[solution_strategy.md]**: Comprehensive guide on problem reformulation, constraints, operational paradigms, game theory, and math techniques.
- **[tricks.rs]**: Useful Rust language-specific snippets and tricks.

### 7. `graph/` - Graph & Tree Algorithms

Graph theory algorithms and tree data processing.

- **`bellman/`**: Bellman-Ford algorithm (Single source, negative cycle detection)
- **`bfs_dijkstra/`**: BFS, Dijkstra's algorithm, 0-1 BFS, and Expanded Graph Dijkstra
- **`bipartite/`**: Bipartite graph detection & coloring
- **`dfs/`**: Depth-First Search (DFS) & Tree DFS
- **`floyd/`**: Floyd-Warshall algorithm (All-pairs shortest paths $O(V^3)$)
- **`functional_graph/`**: Functional Graphs (Out-degree 1 directed graphs & periodicity)
- **`imos/`**: Imos method on graphs
- **`lowest_common_ancestor/`**: Lowest Common Ancestor (LCA)
- **`min_span_tree/`**: Minimum Spanning Tree (MST - Kruskal / Prim)
- **`scc/`**: Strongly Connected Components (SCC)
- **`topological_sort/`**: Topological Sort
- **`tree/`**: Tree diameter, centroid, and path operations

### 8. `greedy/` - Greedy Strategies

Greedy paradigms and verification structures.

- Priority Queue based selection (`priority/`)
- Lexicographical minimization (`lexico/`)
- Order optimization sorting (`order/`)
- Sweepline greedy (`sweepline/`)

### 9. `heuristic/` - Heuristic & Optimization Algorithms

Targeted for AHC (AtCoder Heuristic Contest) and long-running optimization tasks.

- **`simulated_annealing/`**: Simulated Annealing
- **`beam_search/`**: Beam Search
- **`hill_climbing/`**: Hill Climbing
- **`monte_carlo/`**: Monte Carlo methods
- **`bayesian/`**: Bayesian inference & updates
- **`interactive/`**: Interactive optimization

### 10. `idea/` - Problem-Solving Frameworks

Paradigms for shifting perspectives and conceptual problem solving.

- **`subject_object_reverse/`**: Contribution sum ($\sum \sum$ order swapping / subject-object reversal)
- **`think_from_end/`**: Working backwards from the goal
- **`query/`**: Offline query processing, reverse queries (e.g. processing queries backwards with DSU)
- **`game/`**: Game theory (Grundy numbers, Nim, second-player winning strategies)
- **`parity/`**: Parity & Invariants
- **`periodicity/`**: Periodicity & cycle detection
- **`divide_conquer/`**: Divide & Conquer
- **`law_of_conservation/`**: Conservation laws & invariant sums

### 11. `io/` - Input/Output Optimization

- **`fast_io.rs`**: Fast I/O implementation
- **`proconio.rs`**: Usage patterns for AtCoder's `proconio`
- **`file_io.rs`**: File I/O helpers
- **`print.rs`**: Output formatting utilities

### 12. `numbers/` - Math, Number Theory, Combinatorics & Geometry

- **`bit/`**: Bitwise operations (popcount, XOR sum, submask iteration)
- **`calculation/`**: Factorials, powers, $n\text{C}r$
- **`digit/`**: Digit manipulation & sum of digits
- **`geometry/`**: 2D Computational Geometry (Circle intersections, line crossing, slope)
- **`modint/`**: `ModInt` structure for automatic modulo arithmetic
- **`modulo/`**: Modular inverse ($a^{-1} \pmod M$), modular division
- **`prime/`**: Prime testing, Sieve of Eratosthenes, prime factorization
- **`lattice_point/`**: Lattice point scanning & bounds

### 13. `semi_all/` - Semi-Exhaustive Search

Pruned search space, Meet-in-the-middle, and state space reduction.

### 14. `sort/` - Sorting Techniques

- **`angle_sort.rs`**: Angle sorting (atan2 / cross product)
- **`keysort.rs`**: Custom key-based sorting

### 15. `span/` - Array Span & Window Processing

- **`shakutori/`**: Two Pointers (Shakutori method)
- **`prefix/`**: Prefix Sums, 1D/2D Imos Algorithm
- **`rle/`**: Run-Length Encoding (RLE)
- **`sliding_window/`**: Sliding Window Maximum/Minimum
- **`sweepline/`**: Sweepline algorithms

### 16. `strings/` - String Algorithms

- **`hash/`**: Rolling Hash
- **`pattern_matching/`**: KMP algorithm, Z-algorithm
- **`palindrome/`**: Palindrome detection & Manacher's algorithm
- **`parenthesis/`**: Parenthesis matching & sequence operations
- **`subsequences/` / `substrings/`**: Subsequence DP & Substring algorithms

### 17. `structures/` - Data Structures

Advanced data structures essential for competitive programming.

- **`segtree/`**: Segment Tree (RMQ / RSQ)
- **`lazysegtree/`**: Lazy Segment Tree (Range updates & queries)
- **`fenwick/`**: Fenwick Tree / Binary Indexed Tree (BIT)
- **`unionfind/`**: Disjoint Set Union (DSU / Union-Find)
- **`weighted_uf/`**: Potential / Weighted Union-Find
- **`unionfind_binary/`**: Binary / Bipartite Union-Find
- **`sparse_table/`**: Sparse Table (Static $O(1)$ Range Minimum Query)
- **`range_interval_set/`**: Interval Set / Chtholly Tree
- **`binaryheap/`**: Priority Queue / Binary Heap
- **`matrix/`**: Matrix structure (Matrix exponentiation for Fibonacci, etc.)

### 18. `top_coder/` - Generic Templates

Standard templates, enums, interactive I/O helpers, and permutation generators.

---

## ⏱ Time Complexity Estimation Guide

Standard execution time limit in competitive programming (2.0s $\approx 10^8 \sim 10^9$ operations). Reference table for input constraint $N$ and acceptable time complexity:

| Constraint $N$ | Maximum Complexity | Recommended Algorithms & Data Structures |
|---|---|---|
| $N \le 10, 12$ | $O(N!)$ | Permutation search (`all_search/permutation`) |
| $N \le 20, 22$ | $O(2^N)$ | Bitwise brute force (`all_search/bit`), Bit DP (`dp/bit_sales_matching`) |
| $N \le 40$ | $O(2^{N/2})$ | Meet-in-the-middle (`semi_all/`) |
| $N \le 300, 500$ | $O(N^3)$ | Floyd-Warshall (`graph/floyd`), Matrix multiplication (`numbers/matrix`), Interval DP (`dp/spandp`) |
| $N \le 2000, 3000$ | $O(N^2)$ | Double loops, 2D DP, 0-1 Knapsack |
| $N \le 10^5, 2 \times 10^5$ | $O(N \log N)$ | Segment Tree (`structures/segtree`), Sorting (`sort/`), Dijkstra (`graph/bfs_dijkstra`) |
| $N \le 10^6$ | $O(N)$ | Two Pointers (`span/shakutori`), Prefix Sum (`span/prefix`), 1D DP |
| $N \le 10^{12}, 10^{18}$ | $O(\log N), O(\sqrt{N})$ | Binary Search (`binary/`), Prime testing (`numbers/prime`), Matrix Exponentiation (`numbers/modint`) |
