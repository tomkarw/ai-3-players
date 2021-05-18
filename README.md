# 3 person Othello game (now with AI)
Othello game for 3 players, each of them either human or computer.

Computer players implement one of three stategies:
1. Greedy algorithm
2. Weighted sum algorithm
3. Wedge algorithm

# Setting up the game
For now only human games are available.

Simply run:
```
pip install colorama # we use it for fancy terminal coloring
python main.py <player1> <player2> <player3> <minimax-depth>
```
where `player#: human | greedy_ai` and `minimax-depth: <int>`
is the max depth reached in minimax algorithm (3-4 tops for now).

Enjoy!

- TODO: wedge heuristic
- TODO: time optimizations