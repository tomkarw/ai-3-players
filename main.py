from board import Board
from players import HumanPlayer, AIPlayer
from strategies import GreedyStrategy, WeightedSumStrategy, WedgeStrategy


class Game:

    def __init__(self, players_type: list):
        self.players = []
        for turn, player in enumerate(players_type):
            if player == "human":
                self.players.append(HumanPlayer(turn))
            elif player == "greedy_ai":
                self.players.append(AIPlayer(turn, GreedyStrategy()))
            elif player == "weighted_sum_ai":
                self.players.append(AIPlayer(turn, WeightedSumStrategy()))
            elif player == "wedge_ai":
                self.players.append(AIPlayer(turn, WedgeStrategy()))
            else:
                raise NotImplemented("Player type not implemented!")
        self._board = Board()

    def start(self) -> int:
        self._board.setup_three_players()
        game_end = False
        while not game_end:
            for player in self.players:
                player.print_board(self._board)
                row, col = player.get_move(self._board)
                self._board.place(row, col, player.turn)
                if (winner := self._board.check_win()) is not None:
                    return winner


if __name__ == "__main__":
    game = Game(players_type=["human", "human", "human"])
    winner_ = game.start()
    print(f"Congratulations {game.players[winner_].color}, you win!")
