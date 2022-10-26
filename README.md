# Sudoku Solver

Random simple program I create to learn about [Rust](<https://en.wikipedia.org/wiki/Rust_(programming_language)>) while creating [Sudoku](https://en.wikipedia.org/wiki/Sudoku) solver.

## Example

Input the sudoku puzzle or pass from stdin into the program.

```sh
$ cat res/example_unsolved.txt
8 0 0 0 0 0 0 0 0
0 0 3 6 0 0 0 0 0
0 7 0 0 9 0 2 0 0
0 4 0 0 0 7 0 0 0
0 0 0 0 4 5 7 0 0
0 0 0 1 0 0 0 3 0
0 0 1 0 0 0 0 6 8
0 0 8 5 0 0 0 1 0
0 9 0 0 0 0 4 0 0
$ cat res/example_unsolved.txt | cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/sudoku-solver`
8 1 4 7 3 2 6 5 9
9 2 3 6 5 8 1 4 7
5 7 6 4 9 1 2 8 3
3 4 5 2 6 7 8 9 1
1 8 9 3 4 5 7 2 6
2 6 7 1 8 9 5 3 4
7 5 1 9 2 4 3 6 8
4 3 8 5 7 6 9 1 2
6 9 2 8 1 3 4 7 5
```
