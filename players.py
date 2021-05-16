from board import Board
from strategies import Strategy

PLAYER_MAPPINGS = {
    0: "Black",
    1: "White",
    2: "Red",
}


class Player:
    def __init__(self, turn):
        self.turn = turn
        self.color = PLAYER_MAPPINGS[turn]

    def print_board(self, board_: Board) -> None:
        raise NotImplemented

    def get_move(self, board_: Board) -> (int, int):
        raise NotImplemented

    def has_valid_move(self, board_: Board) -> bool:
        return board_.has_valid_move(self.turn)


class HumanPlayer(Player):

    def print_board(self, board_: Board) -> None:
        print(board_)
        print(f"{self.color}'s turn.")

    def get_move(self, board_: Board) -> (int, int):
        move = input("Provide coords as '<int>, <int>': ")
        # loop till valid move
        while True:
            try:
                row, col = map(int, move.split(",", maxsplit=1))
            except ValueError:
                move = input("Invalid input string. Try again: ")
                continue
            valid, reason = board_.validate_placing(row, col, self.turn)
            if not valid:
                move = input(f"{reason} Try again: ")
            else:
                return row, col


class AIPlayer(Player):

    def __init__(self, turn, strategy: Strategy) -> None:
        super().__init__(turn)
        self.strategy = strategy

    def print_board(self, board_: Board) -> None:
        pass  # Robots don't need visuals

    def get_move(self, board_: Board) -> (int, int):
        return self.strategy.get_move(board_)
