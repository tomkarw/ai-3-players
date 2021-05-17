import sys

from board import Board
from players import HumanPlayer, AIPlayer
from strategies import GreedyHeuristic, WeightedSumHeuristic, WedgeHeuristic


class Game:

    def __init__(self, players_type: list, minimax_depth: int):
        self.minimax_depth = minimax_depth
        self.players = []
        for turn, player in enumerate(players_type):
            if player == "human":
                self.players.append(HumanPlayer(turn))
            elif player == "greedy_ai":
                self.players.append(AIPlayer(turn, GreedyHeuristic()))
            elif player == "weighted_sum_ai":
                self.players.append(AIPlayer(turn, WeightedSumHeuristic()))
            elif player == "wedge_ai":
                self.players.append(AIPlayer(turn, WedgeHeuristic()))
            else:
                raise Exception("Invalid player choice")
        self.board = Board()

    def start(self) -> int:
        self.board.setup_three_players()
        game_end = False
        while not game_end:
            players_stuck = True
            for player in self.players:
                player.print_board(self.board)
                if player.has_valid_move(self.board):
                    players_stuck = False
                    row, col = player.get_move(self)
                    self.board.place(row, col, player.turn)
                else:
                    player.print_no_moves()
            if players_stuck:
                return self.board.get_winner()


if __name__ == "__main__":
    if len(sys.argv) != 5:
        print("usage: main.py <player1> <player2> <player3> <minimax-depth>")
    else:
        game = Game(players_type=[sys.argv[1], sys.argv[2], sys.argv[3]], minimax_depth=int(sys.argv[4]))
        winner_ = game.start()
        print(game.board)
        print(f"Congratulations {game.players[winner_].color}, you win!")
