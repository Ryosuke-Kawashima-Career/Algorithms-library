# Sweepline Algorithm

## Oveview

Sweepline is sometimes used for a 2D plane by reducing the number of parameters from 2D to 1D.
It fixes one dimension and move the other.
It is usually used with sorting.

## Generalized Problem Pattern

Sequence `A` is given. You do the following:

- add element `x` to `A`
- Query of the range [l r]

## Generalized Solution

Let `N` be the maximum number of `x`.
Prepare Sequence `B` of length `N` by using Fenwich or Segment Tree.
Combine this with sorting and forseeing queries.
