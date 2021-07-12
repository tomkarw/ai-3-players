# 3 person Othello game (now with simple AI)
Othello game for 3 players, each of them either human or computer.

Computer players implement one of three strategies:
1. Greedy algorithm
2. Weighted sum algorithm
3. Wedge algorithm

# Building the game
On linux disto, make sure you have rustc installed, then run:
```
cargo run <player1> <player2> <player3> <minimax-depth>
```
where `player#: human | greedy_ai | weighted_sum_ai | wedge_ai` is the type of player at `#-th` position and `minimax-depth: <int>`
is the max depth reached in minimax algorithm (5-8 tops for now).

Example:
```
cargo run human weighted_sum_ai greedy_ai 4
```

Enjoy!
