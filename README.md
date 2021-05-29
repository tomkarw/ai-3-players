# 3 person Othello game (now with AI)
Othello game for 3 players, each of them either human or computer.

Computer players implement one of three stategies:
1. Greedy algorithm
2. Weighted sum algorithm
3. Wedge algorithm

# Setting up the game
On linux disto simply run:
```
pip install colorama # we use it for fancy terminal coloring
python main.py <player1> <player2> <player3> <minimax-depth>
```
where `player#: human | greedy_ai | wedge_ai` and `minimax-depth: <int>`
is the max depth reached in minimax algorithm (5-8 tops for now).

If you are on windows, colors are not supported and you don't need to install colorama.

Example:
```
python main.py human weighted_sum_ai greedy_ai 4
```

Enjoy!
