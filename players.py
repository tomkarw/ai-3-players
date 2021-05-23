from copy import deepcopy

from board import Board
from minimax import minimax
from heuristics import Heuristic

PLAYER_MAPPINGS = {
    0: "Black",
    1: "White",
    2: "Red",
}


class Player:
    def __init__(self, turn):
        self.turn = turn
        self.color = PLAYER_MAPPINGS[turn]

    def print_board(self, board: Board) -> None:
        raise NotImplementedError

    def print_no_moves(self) -> None:
        raise NotImplementedError

    def get_move(self, game) -> (int, int):
        raise NotImplementedError

    def has_valid_move(self, board: Board) -> bool:
        return board.has_valid_move(self.turn)


class HumanPlayer(Player):
    def print_board(self, board: Board) -> None:
        print(board)
        print(f"{self.color}'s turn.")

    def print_no_moves(self) -> None:
        print(f"{self.color} has no moves, skipping.")

    def get_move(self, game) -> (int, int):
        move = input("Provide coords as '<int>, <int>': ")
        # loop till valid move
        while True:
            try:
                row, col = map(int, move.split(",", maxsplit=1))
            except ValueError:
                move = input("Invalid input string. Try again: ")
                continue
            valid, reason = game.board.validate_placing(row, col, self.turn)
            if not valid:
                move = input(f"{reason} Try again: ")
            else:
                return row, col


class AIPlayer(Player):
    def __init__(self, turn, heuristic: Heuristic) -> None:
        super().__init__(turn)
        self.heuristic = heuristic

    def print_board(self, board: Board) -> None:
        pass  # Robots don't need visuals

    def print_no_moves(self) -> None:
        pass  # Robots don't need visuals

    def get_move(self, game) -> (int, int):
        best_eval = None
        best_move = None
        valid_moves = game.board.valid_moves(self.turn)
        next_player = (self.turn + 1) % len(game.players)

        for move_row, move_col in valid_moves:
            # TODO(tkarwowski): making this backtrace instead of coping breaks everything
            new_board = deepcopy(game.board)
            new_board.place(move_row, move_col, self.turn)
            evaluation = minimax(
                new_board,
                game.minimax_depth - 1,
                self.turn,
                next_player,
                len(game.players),
                0,
                self.heuristic.evaluate,
                None,
                None,
            )
            best_eval = evaluation if best_eval is None else max(best_eval, evaluation)
            if best_eval == evaluation:
                best_move = (move_row, move_col)

        # TODO(tkarwowski): temporary debug statement (but looks cool)
        # print(game.board)
        return best_move
