# Solution Strategies and Problem-Solving Techniques in Competitive Programming

Solving competitive programming problems generally involves two main steps:

1. **Rephrasing what is requested by the problem** (Equivalent transformation / Reformulation)
2. **Solving by combining known algorithms and data structures**

While essential algorithms and data structures can be learned from textbooks and online resources, "problem reformulation" and "algorithm ideation" come in endless variations and require solving many problems to master.

This article aims to formalize and explain:
- Methods to rephrase requested conditions into equivalent, easily computable tasks
- Mental frameworks for coming up with appropriate algorithms
- Standard ("typical") problem-solving patterns in competitive programming

*(Note: Contains spoilers for some problems)*

---

## Table of Contents

- [1. Thinking from Input Size (Constraints)](#1-thinking-from-input-size-constraints)
  - [1.1. $N \approx 8$](#11-n-approx-8)
  - [1.2. $N \approx 10 \sim 20$](#12-n-approx-10--20)
  - [1.3. $N \approx 30 \sim 40$](#13-n-approx-30--40)
  - [1.4. $N \approx 50$](#14-n-approx-50)
  - [1.5. $N \approx 300 \sim 500$](#15-n-approx-300--500)
  - [1.6. $N \approx 1000$](#16-n-approx-1000)
  - [1.7. $N \approx 3000$](#17-n-approx-3000)
  - [1.8. $N \approx 10^5$](#18-n-approx-105)
  - [1.9. $N$ Doesn't Fit in 64-bit Integer](#19-n-doesnt-fit-in-64-bit-integer)
  - [1.10. Small Modulo Given](#110-small-modulo-given)
  - [1.11. Meaningful Constants in the Problem Statement](#111-meaningful-constants-in-the-problem-statement)
- [2. Fundamental: Can It Be Solved by Exhaustive Search?](#2-fundamental-can-it-be-solved-by-exhaustive-search)
- [3. Fundamental: Do Not Try to Do Two or More Things at Once](#3-fundamental-do-not-try-to-do-two-or-more-things-at-once)
  - [3.1. Decompose the Problem (Consider Simpler Cases)](#31-decompose-the-problem-consider-simpler-cases)
  - [3.2. Fix Parameters / Variables](#32-fix-parameters--variables)
- [4. Leveraging Symmetry](#4-leveraging-symmetry)
  - [4.1. Equal Symmetric Counterparts: Compute Total and Divide by 2](#41-equal-symmetric-counterparts-compute-total-and-divide-by-2)
  - [4.2. Reduce to 1D Problem Using Coordinate Symmetry](#42-reduce-to-1d-problem-using-coordinate-symmetry)
- [5. Operation Problems](#5-operation-problems)
  - [5.1. Focus on Invariants](#51-focus-on-invariants)
  - [5.2. Focus on Parity](#52-focus-on-parity)
  - [5.3. Can the Order of Operations Be Swapped?](#53-can-the-order-of-operations-be-swapped)
  - [5.4. What If We Consider Operations in Reverse Order?](#54-what-if-we-consider-operations-in-reverse-order)
  - [5.5. Can Operations Be Reverted?](#55-can-operations-be-reverted)
  - [5.6. Example Problems](#56-example-problems)
- [6. Counting Problems](#6-counting-problems)
  - [6.1. Group States with Dynamic Programming to Speed Up Brute Force](#61-group-states-with-dynamic-programming-to-speed-up-brute-force)
  - [6.2. Combinations with Average $K$](#62-combinations-with-average-k)
- [7. Interval Problems](#7-interval-problems)
  - [7.1. Precompute Cumulative Information from Both Left and Right](#71-precompute-cumulative-information-from-both-left-and-right)
  - [7.2. Adding Values to Intervals](#72-adding-values-to-intervals)
  - [7.3. Deleting, Compressing, or Composing Intervals](#73-deleting-compressing-or-composing-intervals)
  - [7.4. Arithmetic Progression Addition: Look at Differences](#74-arithmetic-progression-addition-look-at-differences)
  - [7.5. Interval Reversal Problems](#75-interval-reversal-problems)
- [8. Game Theory](#8-game-theory)
  - [8.1. Impartial Games: Grundy Numbers](#81-impartial-games-grundy-numbers)
  - [8.2. Retrograde Analysis (Working Backwards from the End)](#82-retrograde-analysis-working-backwards-from-the-end)
  - [8.3. Continuing the Same Move is Optimal](#83-continuing-the-same-move-is-optimal)
  - [8.4. One Side Maximizes, Other Minimizes](#84-one-side-maximizes-other-minimizes)
- [9. Counting Objects Satisfying Constraints](#9-counting-objects-satisfying-constraints)
  - [9.1. Consider Complementary Events When Direct Counting is Hard](#91-consider-complementary-events-when-direct-counting-is-hard)
- [10. Construction Problems](#10-construction-problems)
  - [10.1. "Impossible Cases" Might Not Exist](#101-impossible-cases-might-not-exist)
  - [10.2. Construction Strategy at Upper Bound Limit May Apply to All Cases](#102-construction-strategy-at-upper-bound-limit-may-apply-to-all-cases)
  - [10.3. "Finish Within $2N$ Operations": Reach a Special State Within $N$ Steps](#103-finish-within-2n-operations-reach-a-special-state-within-n-steps)
  - [10.4. Example Problems](#104-example-problems)
- [11. Finding the $K$-th Value](#11-finding-the-k-th-value)
  - [11.1. When Element Values Are Small](#111-when-element-values-are-small)
  - [11.2. Binary Search for the $K$-th Value](#112-binary-search-for-the-k-th-value)
  - [11.3. Example Problems](#113-example-problems)
- [12. XOR Problems](#12-xor-problems)
  - [12.1. View XOR as "Addition Without Carry"](#121-view-xor-as-addition-without-carry)
  - [12.2. Consider XOR Bit by Bit](#122-consider-xor-bit-by-bit)
  - [12.3. XORing the Same Value Twice Cancels Out](#123-xoring-the-same-value-twice-cancels-out)
  - [12.4. Example Problems](#124-example-problems)
- [13. Manhattan Distance](#13-manhattan-distance)
  - [13.1. Rotate Coordinates by 45 Degrees](#131-rotate-coordinates-by-45-degrees)
  - [13.2. Difference Minimization Uses Median](#132-difference-minimization-uses-median)
- [14. Graph Theory](#14-graph-theory)
  - [14.1. Experiment with Representative Graph Structures](#141-experiment-with-representative-graph-structures)
  - [14.2. Utilize the Fact that Trees Are Bipartite Graphs](#142-utilize-the-fact-that-trees-are-bipartite-graphs)
  - [14.3. Consider the Diameter of a Tree](#143-consider-the-diameter-of-a-tree)
- [15. Finding Max/Min Satisfying Conditions](#15-finding-maxmin-satisfying-conditions)
  - [15.1. Binary Search for Max/Min Satisfying Conditions](#151-binary-search-for-maxmin-satisfying-conditions)
  - [15.2. Greedy Approach from the Side with Fewer Choices](#152-greedy-approach-from-the-side-with-fewer-choices)
  - [15.3. Example Problems](#153-example-problems)
- [16. Focusing on Parity](#16-focusing-on-parity)
  - [16.1. $|x - y|$ and Difference Calculations](#161-x---y-and-difference-calculations)
  - [16.2. Check Parity via XOR](#162-check-parity-via-xor)
  - [16.3. Analyze Odd Numbers via Parity](#163-analyze-odd-numbers-via-parity)
- [17. Maximum Matching](#17-maximum-matching)
  - [17.1. Bipartite Matching Using Network Flow](#171-bipartite-matching-using-network-flow)
  - [17.2. Greedy Matching When Structure Permits](#172-greedy-matching-when-structure-permits)
- [18. Miscellaneous Techniques](#18-miscellaneous-techniques)
  - [18.1. Sum of Products $\to$ Product of Sums via Distributive Law](#181-sum-of-products-to-product-of-sums-via-distributive-law)
  - [18.2. Model 2D Coordinates as Bipartite Graphs](#182-model-2d-coordinates-as-bipartite-graphs)
  - [18.3. Model Order Relationships as Directed Acyclic Graphs (DAGs)](#183-model-order-relationships-as-directed-acyclic-graphs-dags)
  - [18.4. Ternary Search for Extrema of Convex Functions](#184-ternary-search-for-extrema-of-convex-functions)
  - [18.5. Monotonic Property on Sequence Indices: Two Pointers](#185-monotonic-property-on-sequence-indices-two-pointers)
  - [18.6. Geometric Progressions and Base-$N$ Numbers: Analyze Remainders](#186-geometric-progressions-and-base-n-numbers-analyze-remainders)
  - [18.7. Parenthesis Sequences: Model as Mountain Paths](#187-parenthesis-sequences-model-as-mountain-paths)

---

## 1. Thinking from Input Size (Constraints)

Checking constraint bounds is often the very first step in deciding an approach. Sometimes guessing the intended time complexity from constraints reveals the algorithm before fully analyzing the problem logic.

### 1.1. $N \approx 8$
- Likely $O(N!)$ factorial search (permutations search).

### 1.2. $N \approx 10 \sim 20$
- Likely $O(2^N)$ exponential search (bit brute force).
- For small $N$, Bit DP may work in $O(N^2 2^N)$ complexity. Inclusion-Exclusion Principle is also common.
- **Example**: [Keyence 2020 D - Swap and Flip](https://atcoder.jp/contests/keyence2020/tasks/keyence2020_d) ($N \le 18$, Bit DP).
- **Example**: [ABC152 F - Tree and Constraints](https://atcoder.jp/contests/abc152/tasks/abc152_f) ($M \le 20$ constraints, Inclusion-Exclusion).
- **Example**: [Typical 90 002 - Encyclopedia of Parentheses](https://atcoder.jp/contests/typical90/tasks/typical90_b).

### 1.3. $N \approx 30 \sim 40$
- Meet-in-the-middle (Split and List) can reduce $O(2^N)$ to $O(N 2^{N/2})$.
- **Example**: [AGC 026 C - String Coloring](https://atcoder.jp/contests/agc026/tasks/agc026_c).

### 1.4. $N \approx 50$
- $O(N^4)$ algorithm will pass within execution time limit.
- **Example**: [ARC 060 C - Takahashi and Cards](https://atcoder.jp/contests/arc060/tasks/arc060_a) (DP in $O(N^3 X)$ or $O(N^2 X)$).

### 1.5. $N \approx 300 \sim 500$
- $O(N^3)$ algorithms will pass (e.g. Range DP, Floyd-Warshall).

### 1.6. $N \approx 1000$
- $O(N^2 \log N)$ or $O(N^2)$ algorithms will pass.
- Speeding up $O(N^4)$ via Meet-in-the-middle or binary search.
- **Example**: [JOI 2008 Final C - Darts](https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_c) (Combine pairs into $O(N^2 \log N)$ search).
- **Example**: [ABC 034 D - Salt Water](https://atcoder.jp/contests/abc034/tasks/abc034_d) (Maximizing average via binary search).

### 1.7. $N \approx 3000$
- $O(N^2)$ algorithms will pass.

### 1.8. $N \approx 10^5$
- The most common constraint. Naive solutions taking $O(N^2)$ TLE, requiring optimization to $O(N \log N)$ or $O(N)$.
  - Sorting in $O(N \log N)$
  - Binary search / `BTreeSet` / `HashMap` inside a single loop: $O(N \log N)$
  - Dynamic programming updating $N \times \text{constant}$ array
  - Binary search on answer with $O(N \log N)$ check: $O(N \log^2 N)$
- **Example**: [ABC138 E - Strings of Impurity](https://atcoder.jp/contests/abc138/tasks/abc138_e) (Binary search on index positions).
- **Example**: [ABC023 D - King's Inspection](https://atcoder.jp/contests/abc023/tasks/abc023_d) (Binary search on answer).

### 1.9. $N$ Doesn't Fit in 64-bit Integer
- When handling values $> 10^{18}$, process input **digit by digit** (often using string input & Digit DP).
- **Example**: [ABC135 D - Digits Parade](https://atcoder.jp/contests/abc135/tasks/abc135_d).
- **Example**: [ABC154 E - Almost Everywhere Zero](https://atcoder.jp/contests/abc154/tasks/abc154_e).

### 1.10. Small Modulo Given
- When mod is relatively small (e.g. a few thousands instead of $10^9+7$), tracking remainders as DP states works well.

### 1.11. Meaningful Constants in the Problem Statement
- If an unusual constant is given (e.g. 13 in ABC135 D), ask *why that specific number is chosen*. It often indicates small DP state space or special number-theoretic properties.

---

## 2. Fundamental: Can It Be Solved by Exhaustive Search?

Computers easily execute $\approx 10^7$ operations per second. If the search space is small, writing a direct brute force search (nested loops, recursion, bitmask, permutations) is fast to code and bug-free.

Even for complex problems, estimating the complexity of brute force helps invent optimizations like DP or binary search.

---

## 3. Fundamental: Do Not Try to Do Two or More Things at Once

Handling multiple variables or constraints simultaneously leads to confusion.

### 3.1. Decompose the Problem (Consider Simpler Cases)
- Break down complex settings:
  - If inputs are integers, analyze positive-only, negative-only, or zero cases separately.
  - Split 2D coordinate problems into independent 1D problems for $X$ and $Y$.
- **Example**: [ARC 107 C - Shuffle Permutation](https://atcoder.jp/contests/arc107/tasks/arc107_c) (Rows and columns shuffle independently).
- **Example**: [ABC099 C - Strange Bank](https://atcoder.jp/contests/abc099/tasks/abc099_c) (Separate $6^i$ and $9^i$ coin payments).
- **Example**: [ARC 086 D - Non-decreasing](https://atcoder.jp/contests/arc086/tasks/arc086_b) (Handle positive vs. negative dominance separately).

### 3.2. Fix Parameters / Variables
- When $O(N^2)$ brute force considers two variables, **fixing one variable** often enables computing the second variable in $O(1)$ or $O(\log N)$.
- When picking 3 elements ($A, B, C$), **fix the middle element $B$**.
- When partitioning into 4 parts (3 dividers), **fix the center divider**.
- **Example**: [ABC077 C - Snuke Festival](https://atcoder.jp/contests/abc077/tasks/arc084_a) (Fix middle element $B$).
- **Example**: [ARC 100 D - Equal Cut](https://atcoder.jp/contests/arc100/tasks/arc100_b) (Fix center divider).
- **Example**: [ABC104 D - We Love ABC](https://atcoder.jp/contests/abc104/tasks/abc104_d) (Fix letter 'B').

---

## 4. Leveraging Symmetry

### 4.1. Equal Symmetric Counterparts: Compute Total and Divide by 2
When the target value equals its symmetric counterpart, computing their combined sum is often easier than computing one side directly. Calculate total sum first, then divide by 2.
- **Example**: [ARC 106 D - Powers](https://atcoder.jp/contests/arc106/tasks/arc106_d).

### 4.2. Reduce to 1D Problem Using Coordinate Symmetry
Problems involving squares on 2D grids often feature independent $X$ and $Y$ symmetry, turning 2D problems into two independent 1D problems.
- **Example**: [HHKB 2020 D - Squares](https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_d).

---

## 5. Operation Problems

### 5.1. Focus on Invariants
Identify values that remain constant under allowed operations (e.g. sum of elements remains constant).

### 5.2. Focus on Parity
Check if operations preserve parity (even/odd) or flip it predictably.

### 5.3. Can the Order of Operations Be Swapped?
If operation order does not affect the final result, apply operations in the most convenient order (e.g., greedy order).

### 5.4. What If We Consider Operations in Reverse Order?
Working backwards from the target state to the initial state often simplifies logic.

### 5.5. Can Operations Be Reverted?
If operations are reversible, group reachable states into equivalence classes.

### 5.6. Example Problems
- **Example**: [ABC136 E - Max GCD](https://atcoder.jp/contests/abc136/tasks/abc136_e) (Total sum invariant $\implies$ answer divides total sum).
- **Example**: [ABC127 D - Integer Cards](https://atcoder.jp/contests/abc127/tasks/abc127_d) (Operation order independent $\implies$ process largest replacements first).
- **Example**: [ABC093 C - Same Integers](https://atcoder.jp/contests/abc093/tasks/arc094_a) (Parity invariant under $+2$ operations).
- **Example**: [AGC 037 C - Numbers on a Circle](https://atcoder.jp/contests/agc037/tasks/agc037_c) (Work backwards from final values).
- **Example**: [ARC 071 E - TrBBnsformBBtion](https://atcoder.jp/contests/arc071/tasks/arc071_c) (Reversibility & equivalence classes).

---

## 6. Counting Problems

### 6.1. Group States with Dynamic Programming to Speed Up Brute Force
Identify which state attributes can be aggregated into DP states to avoid redundant search paths.

### 6.2. Combinations with Average $K$
Finding combinations of elements with average $K$ is equivalent to:
1. Combinations with $\sum A_i = K \times \text{count}$
2. Subtracting $K$ from all elements and finding combinations with sum $= 0$.
- **Example**: [ARC 060 C - Takahashi and Cards](https://atcoder.jp/contests/arc060/tasks/arc060_a).

---

## 7. Interval Problems

### 7.1. Precompute Cumulative Information from Both Left and Right
Precomputing prefix and suffix values (sums, min/max, GCD) allows evaluating queries excluding a single element in $O(1)$.
- **Example**: [ABC098 C - Attention](https://atcoder.jp/contests/abc098/tasks/arc098_a) (Prefix 'W' count & Suffix 'E' count).
- **Example**: [ABC125 C - GCD on Blackboard](https://atcoder.jp/contests/abc125/tasks/abc125_c) (Prefix & Suffix GCD).

### 7.2. Adding Values to Intervals
Use Imos algorithm (Difference Array) or Fenwick Tree (BIT) with range addition.

### 7.3. Deleting, Compressing, or Composing Intervals
Use Range DP ($O(N^3)$).

### 7.4. Arithmetic Progression Addition: Look at Differences
Adding an AP with common difference $d$ to an interval changes consecutive difference values by $+d$ everywhere except at the boundaries.
- **Example**: [AGC 010 B - Boxes](https://atcoder.jp/contests/agc010/tasks/agc010_b).

### 7.5. Interval Reversal Problems
- Applying reversal twice cancels out.
- Reversal order does not matter.
- Reversing $[l_1, r_1]$ and $[l_2, r_2]$ is equivalent to reversing $[l_1, r_2]$ and $[l_2, r_1]$.
- **Example**: [JSC2019 Qual C - Cell Inversion](https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_c).

---

## 8. Game Theory

### 8.1. Impartial Games: Grundy Numbers
For impartial games under normal play convention, compute Grundy numbers (Nim-values) via Mex and XOR sum.

### 8.2. Retrograde Analysis (Working Backwards from the End)
Determine winning/losing states by analyzing backwards from terminal states.

### 8.3. Continuing the Same Move is Optimal
- **Example**: [ABC 027 C - Double Doubling Game](https://atcoder.jp/contests/abc027/tasks/abc027_c).

### 8.4. One Side Maximizes, Other Minimizes
- **Case distinction**: Determine Player 1's best moves, then evaluate Player 2's optimal counter-moves.
- **Bounding logic**: If Max-player guarantees outcome $\ge X$ and Min-player guarantees outcome $\le X$, optimal outcome is exactly $X$.
- **Example**: [ARC 094 E - Tozan and Gezan](https://atcoder.jp/contests/arc094/tasks/arc094_c).
- **Example**: [ABC 078 D - ABS](https://atcoder.jp/contests/abc078/tasks/arc085_b).

---

## 9. Counting Objects Satisfying Constraints

### 9.1. Consider Complementary Events When Direct Counting is Hard
$\text{Count}(\text{Condition } A) = \text{Total} - \text{Count}(\text{NOT } A)$.  
Particularly, "at least one" conditions become "exactly zero" under negation.
- **Example**: [ABC152 F - Tree and Constraints](https://atcoder.jp/contests/abc152/tasks/abc152_f).

---

## 10. Construction Problems

### 10.1. "Impossible Cases" Might Not Exist
Output `-1 if impossible` prompts might be red herrings—a valid construction may exist for all valid inputs.

### 10.2. Construction Strategy at Upper Bound Limit May Apply to All Cases
A construction pattern tailored for the maximum constraint limit (e.g., exactly $N$ elements) often generalizes to all inputs.

### 10.3. "Finish Within $2N$ Operations": Reach a Special State Within $N$ Steps
Execute $N$ setup operations to reach a structured intermediate state, then solve in $\le N$ operations.

### 10.4. Example Problems
- **Example**: [ABC 068 D - Decrease (Contestant ver.)](https://atcoder.jp/contests/abc068/tasks/arc079_b) (Constructing with fixed size 50 array).
- **Example**: [ARC 086 D - Non-decreasing](https://atcoder.jp/contests/arc086/tasks/arc086_b).

---

## 11. Finding the $K$-th Value

### 11.1. When Element Values Are Small
Use Fenwick Tree (BIT) / Segment Tree with binary lifting to query the $K$-th smallest element in $O(\log N)$.

### 11.2. Binary Search for the $K$-th Value
Rephrase:
> "$K$-th smallest value is $X$"  
> $\iff$ "Count of elements $\le X-1$ is $< K$, and count of elements $\le X$ is $\ge K$"  
> $\iff$ "Smallest $X$ such that count of elements $\le X$ is $\ge K$"

Binary search on $X$ requires $O(\log (\text{Range}))$ check calls to count elements $\le X$.

#### Finding the Median ($(N/2 + 1)$-th element)
Finding the median is equivalent to binary searching for the smallest $X$ with $\ge N/2 + 1$ elements $\le X$.

### 11.3. Example Problems
- **Example**: [ARC037 C - Billion Calculation](https://atcoder.jp/contests/arc037/tasks/arc037_c).
- **Example**: [ABC155 D - Pairs](https://atcoder.jp/contests/abc155/tasks/abc155_d).

---

## 12. XOR Problems

### 12.1. View XOR as "Addition Without Carry"
XOR is bitwise addition without carry:
$$a + b = (a \oplus b) + 2(a \text{ \& } b)$$
Addition equals XOR if and only if no carry occurs ($(a \text{ \& } b) = 0$).

### 12.2. Consider XOR Bit by Bit
XOR operations on bit position $k$ are completely independent of other bit positions. Solve per bit and sum up $2^k \times \text{count}$.

### 12.3. XORing the Same Value Twice Cancels Out
$$a \oplus a = 0 \quad \text{and} \quad a \oplus a \oplus b = b$$

### 12.4. Example Problems
- **Example**: [ARC021 B - Your Numbers are XORed...](https://atcoder.jp/contests/arc021/tasks/arc021_2).
- **Example**: [ABC129 E - Sum Equals Xor](https://atcoder.jp/contests/abc129/tasks/abc129_e).
- **Example**: [ABC098 D - Xor Sum 2](https://atcoder.jp/contests/abc098/tasks/arc098_b).
- **Example**: [ABC147 D - Xor Sum 4](https://atcoder.jp/contests/abc147/tasks/abc147_d).
- **Example**: [ABC121 D - XOR World](https://atcoder.jp/contests/abc121/tasks/abc121_d).
- **Example**: [Codeforces R628 Div2 D - Ehab the Xorcist](https://codeforces.com/contest/1325/problem/D).

---

## 13. Manhattan Distance

### 13.1. Rotate Coordinates by 45 Degrees
Transform $(x, y) \to (X, Y) = (x + y, x - y)$.  
Manhattan distance $\le d$ becomes Chebyshev distance box $[X - d, X + d] \times [Y - d, Y + d]$, decoupling $X$ and $Y$ bounds.
- **Example**: [ABC018 C - Diamond Counting](https://atcoder.jp/contests/abc018/tasks/abc018_3).

### 13.2. Difference Minimization Uses Median
The value of $x$ minimizing $\sum_{i=1}^n |A_i - x|$ is the **median** of sequence $A$.
- **Example**: [ABC102 C - Linear Approximation](https://atcoder.jp/contests/abc102/tasks/arc100_a).

---

## 14. Graph Theory

### 14.1. Experiment with Representative Graph Structures
Test hypotheses on canonical graph structures:
- Path Graph
- Star Graph
- Complete Graph ($K_N$)
- Complete Bipartite Graph
- Path Graph with few extra edges
- **Example**: [ABC131 E - Friendships](https://atcoder.jp/contests/abc131/tasks/abc131_e).

### 14.2. Utilize the Fact that Trees Are Bipartite Graphs
Vertices in trees can always be 2-colored. Even distance steps remain in the same color class; odd distance steps switch classes.
- **Example**: [Hitachi 2020 C - ThREE](https://atcoder.jp/contests/hitachi2020/tasks/hitachi2020_c).

### 14.3. Consider the Diameter of a Tree
The diameter of a tree often reduces tree problems to path graph problems.
- **Example**: [AGC 033 C - Removing Coins](https://atcoder.jp/contests/agc033/tasks/agc033_c).

---

## 15. Finding Max/Min Satisfying Conditions

### 15.1. Binary Search for Max/Min Satisfying Conditions
Use binary search on answer when optimizing a monotonic predicate $C(x)$:
- Maximizing/minimizing averages
- Maximizing the minimum element (Max-Min)
- Minimizing the maximum element (Min-Max)

### 15.2. Greedy Approach from the Side with Fewer Choices
When options shrink over time or index, make greedy choices starting from the end (strictest constraint side).
- **Example**: [ABC137 D - Summer Vacation](https://atcoder.jp/contests/abc137/tasks/abc137_d) (Work backwards from the final deadline day).
- **Example**: [ABC134 D - Preparing Boxes](https://atcoder.jp/contests/abc134/tasks/abc134_d) (Process backwards from box $N$ to box 1).
- **Example**: [DISCO 2016 Final B - DDPC Special Buffet II](https://atcoder.jp/contests/discovery2016-final).

---

## 16. Focusing on Parity

### 16.1. $|x - y|$ and Difference Calculations
Modulo 2 relations:
$$|x - y| \equiv x \oplus y \equiv x + y \pmod 2$$

### 16.2. Check Parity via XOR
Use XOR to track parity state changes efficiently.

### 16.3. Analyze Odd Numbers via Parity
- **Example**: [AGC 043 B - 123 Triangle](https://atcoder.jp/contests/agc043/tasks/agc043_b).

---

## 17. Maximum Matching

### 17.1. Bipartite Matching Using Network Flow
Model bipartite matching with Dinic's or Ford-Fulkerson algorithm ($O(E \sqrt{V})$ or $O(VE)$).

### 17.2. Greedy Matching When Structure Permits
If bipartite graph construction is too large, use greedy matching from the side with fewer choices.
- **Example**: [AGC 029 B - Powers of two](https://atcoder.jp/contests/agc029/tasks/agc029_b).

---

## 18. Miscellaneous Techniques

### 18.1. Sum of Products $\to$ Product of Sums via Distributive Law
Transform sum of products into product of sums to reduce $O(N^k)$ to $O(N \cdot k)$:
$$\sum_{a=1}^A \sum_{b=1}^B \sum_{c=1}^C a b c = \left( \sum_{a=1}^A a \right) \left( \sum_{b=1}^B b \right) \left( \sum_{c=1}^C c \right)$$
- **Example**: [ARC 107 A - Simple Math](https://atcoder.jp/contests/arc107/tasks/arc0107_a).

### 18.2. Model 2D Coordinates as Bipartite Graphs
- Left vertex set: $X$-coordinates
- Right vertex set: $Y$-coordinates
- Edge $(x, y)$: point at coordinate $(x, y)$
- **Example**: [ABC 131 F - Must Be Rectangular!](https://atcoder.jp/contests/abc131/tasks/abc131_f) (Connected components form complete bipartite graphs).

### 18.3. Model Order Relationships as Directed Acyclic Graphs (DAGs)
Partial order relations form DAGs $\implies$ use Topological Sort & Cycle Detection.
- **Example**: [ABC139 E - League](https://atcoder.jp/contests/abc139/tasks/abc139_e).

### 18.4. Ternary Search for Extrema of Convex Functions
Query minimum/maximum of unimodal / convex functions via Ternary Search or Golden-Section Search.
- **Example**: [ARC054 B - Moore's Law](https://atcoder.jp/contests/arc054/tasks/arc054_b).

### 18.5. Monotonic Property on Sequence Indices: Two Pointers
When valid right boundary $r = f(l)$ is monotonic with respect to $l$, use Two Pointers (Shakutori) for $O(N)$ runtime.
- **Example**: [ABC098 D - Xor Sum 2](https://atcoder.jp/contests/abc098/tasks/arc098_b).

### 18.6. Geometric Progressions and Base-$N$ Numbers: Analyze Remainders
Analyze remainders to determine digits from lowest to highest significance.
- **Example**: [ABC105 C - Base -2 Number](https://atcoder.jp/contests/abc105/tasks/abc105_c).

### 18.7. Parenthesis Sequences: Model as Mountain Paths
Model `'('` as $+1$ step up, `')'` as $-1$ step down.  
A valid parenthesis sequence requires:
1. Height $\ge 0$ at all prefix points.
2. Final height $= 0$ at the end.
