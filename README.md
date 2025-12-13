# AoC2025-Rust
My solutions to the Advent of Code 2025 puzzles

### Thoughts
The solutions are run all together in the [main](src/main.rs) file,
compiled down to a single optimized executable.
<br>
The answers are obviously specific to the inputs, which is why I made it so that anyone
can build and run the project with their own inputs (although I only tested the program on Linux).

This was the first time I was actually able to complete the [**Advent of Code**](https://adventofcode.com/2025)
during the event, every puzzle solved within a day (an average time for part 2 of 5.5 hours),
and while I did sketch most solutions using Python, I am still proud I was able to come up with
quick and dirty code to get the answer, before going and optimizing here in Rust.

Just like [AoC2024](https://github.com/massimopavoni/AoC2025-Rust), I was able to keep the
total execution time of all solutions under 40ms, although that might have not been the case
if the number of problems wasn't cut in half: in my opinion, some of them are more computationally expensive
in general, when compared to past events' problems, which is probably a result of adapting the difficulty curve
to get steeper earlier.

Nonetheless, a great learning experience, as always.

### Days
No write-ups, just the usual collection of days and concepts reviewed, studied and learned.

1. [Secret Entrance](src/secret_entrance.rs) ->
   rotation simulation,
   [modular arithmetic](https://en.wikipedia.org/wiki/Modular_arithmetic)

2. [Gift Shop](src/gift_shop.rs) ->
   number patterns generation,
   [series](https://en.wikipedia.org/wiki/Series_(mathematics)),
   [Möbius function](https://en.wikipedia.org/wiki/M%C3%B6bius_function)

3. [Lobby](src/lobby.rs) ->
   consecutive digits maximum number,
   [stacks](https://en.wikipedia.org/wiki/Stack_(abstract_data_type))

4. [Printing Department](src/printing_department.rs) ->
   [BFS](https://en.wikipedia.org/wiki/Breadth-first_search)

5. [Cafeteria](src/cafeteria.rs) ->
   ranges merging

6. [Trash Compactor](src/trash_compactor.rs) ->
   weird creatures arithmetic,
   bytes [matrix transposition](https://en.wikipedia.org/wiki/Transpose)

7. [Laboratories](src/laboratories.rs) ->
   [memoization](https://en.wikipedia.org/wiki/Memoization),

8. [Playground](src/playground.rs) ->
   [Kruskal's algorithm](https://en.wikipedia.org/wiki/Kruskal%27s_algorithm),
   graph [MST](https://en.wikipedia.org/wiki/Minimum_spanning_tree),
   [DSU](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) data structure

9. [Movie Theater](src/movie_theater.rs) ->
   polygons and rectangles geometry

10. [Factory](src/factory.rs) ->
    [power set](https://en.wikipedia.org/wiki/Power_set),
    linear algebra,
    [Gaussian elimination](https://en.wikipedia.org/wiki/Gaussian_elimination),
    free variables,
    [linear optimization](https://en.wikipedia.org/wiki/Linear_programming),
    [ILP](https://en.wikipedia.org/wiki/Integer_programming),
    [*microlp*](https://crates.io/crates/microlp)

11. [Reactor](src/reactor.rs) ->
    [directed graphs](https://en.wikipedia.org/wiki/Directed_graph),
    [*pathfinding*](https://crates.io/crates/pathfinding)

12. [Christmas Tree Farm](src/christmas_tree_farm.rs) ->
    [packing problems](https://en.wikipedia.org/wiki/Packing_problems),
    trivial packing solutions

One additional thing I learned during **AoC2025** is [*lexical-core*](https://crates.io/crates/lexical-core) parsing.
