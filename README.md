# Solver for clock puzzle in Final Fantasy XIII-2

A simple brute-force solver for the clock puzzles in Final Fantasy XIII-2.

## Usage

Enter the clock values you see, starting from the topmost (12 o'clock position)
and going clockwise.

The result will be the clock positions you should press, in order.

For example, if the clock has 10 numbers, then `0` will be the topmost one, and
`9` will be to the immediate left (one space counterclockwise),
and `1` will be to the right (one space clockwise).

Example for a 12-position clock from the game:

    $ cargo run 1 5 3 4 2 6 2 2 4 5 5 5

    Result: 10, 3, 7, 9, 2, 5, 11, 4, 6, 8, 0, 1
