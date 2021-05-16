from board import Board, ForbiddenMove
from strategies import Strategy


PLAYER_MAPPINGS = {
    0: "white",
    1: "black",
    2: "red",
}


class Player:
    def print_board(self, board_: Board) -> None:
        raise NotImplemented

    def get_move(self, board_: Board) -> (int, int):
        raise NotImplemented


class HumanPlayer:

    def __init__(self, turn):
        self.turn = turn
        self.color = PLAYER_MAPPINGS[turn]

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
            try:
                board_.validate_placing(row, col, self.turn)
            except ForbiddenMove as reason:
                move = input(f"{reason}. Try again: ")
            else:
                return row, col


class AIPlayer:
    color: str
    turn: int
    strategy: Strategy

    def __init__(self, turn, strategy) -> None:
        self.turn = turn
        self.color = PLAYER_MAPPINGS[turn]
        self.strategy = strategy

    def print_board(self, board_: Board) -> None:
        pass  # Robots don't need visuals

    def get_move(self, board_: Board) -> (int, int):
        return self.strategy.get_move(board_)
