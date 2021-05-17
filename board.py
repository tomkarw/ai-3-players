import itertools
from dataclasses import dataclass
from typing import Union

from colorama import Back, Style

EMPTY = -1

COLOR_MAPPING = {
    -1: "",
    0: Back.BLACK,
    1: Back.WHITE,
    2: Back.RED,
}


class ForbiddenMove(Exception):
    pass


@dataclass
class Board:
    """
    Stateful board of Othello game
    """
    board: list[list[int]]
    rows: int = 9
    columns: int = 9

    def __init__(self) -> None:
        super().__init__()
        self.board = [[EMPTY for _ in range(self.columns)] for _ in range(self.rows)]

    def topbot_line(self) -> str:
        return "  " + Back.GREEN + " ".join(["+"] + (["-  "] * self.columns)[:-1] + [f"- +{Style.RESET_ALL}\n"])

    def __str__(self):
        output = ""
        # column numbering
        output += "    " + "   ".join(map(str, range(self.columns))) + "\n"
        # top line
        output += self.topbot_line()
        # iterate over all but last row
        for row_num, row in enumerate(self.board):
            output += f"{row_num} {Back.GREEN}|"
            for cell in row:
                output += self.print_disc(cell) + "|"
            output += f"{Style.RESET_ALL}\n"
            if row_num != self.rows - 1:
                output += "  " + Back.GREEN + " ".join([" "] + ["-  "] * self.columns) + Style.RESET_ALL + "\n"
        # bottom line
        output += self.topbot_line()
        return output

    @staticmethod
    def print_disc(cell):
        color = COLOR_MAPPING[cell]
        return f"{color or ''}   {Back.GREEN}"

    def setup_three_players(self):
        """
        Create starting position for 3 player game
        """
        self.board[3][3] = self.board[3][5] = self.board[4][4] = 0  # white
        self.board[3][4] = self.board[5][3] = self.board[5][5] = 1  # black
        self.board[4][3] = self.board[4][5] = self.board[5][4] = 2  # red

    def place(self, row: int, column: int, player: int):
        self.validate_placing(row, column, player)
        to_flip = self.would_flip(row, column, player)
        for x, y in to_flip:
            self.board[x][y] = player
        self.board[row][column] = player

    def validate_placing(self, row: int, column: int, player: int) -> (bool, str):
        """
        Make sure move is valid.
        If not raise ForbiddenMove exception with human readable reason why the move is not allowed.
        """
        # TODO(tkarwowski)
        # outside bounds
        if row < 0 or row >= self.rows or column < 0 or column >= self.columns:
            return False, "Placement out of bounds."

        # square taken
        if self.board[row][column] != EMPTY:
            return False, "Square taken."

        # square not adjacent to any current discs
        is_adjacent = False
        for y in [row - 1, row, row + 1]:
            for x in [column - 1, column, column + 1]:
                # outside of bounds, skip
                if y < 0 or y >= self.rows or x < 0 or x >= self.columns:
                    continue
                # this is handled in square taken, but double-check
                if row == y and column == x:
                    continue
                # check if square is taken
                if self.board[y][x] != EMPTY:
                    is_adjacent = True
                    break
            if is_adjacent:
                break
        if not is_adjacent:
            return False, "New disc must be adjacent to some existing one."

        # placing must flip at least one opposing disk
        if not self.would_flip(row, column, player):
            return False, "Move won't flip any disks."

        return True, ""

    def would_flip(self, row, column, player) -> list[(int, int)]:
        flips = []
        for dy, dx in itertools.product([-1, 0, 1], [-1, 0, 1]):
            visited_discs = []
            if dy == dx == 0:
                continue
            new_row = row + dy
            new_col = column + dx
            # stop when we reach board border
            while not (new_row < 0 or new_row >= self.rows or new_col < 0 or new_col >= self.columns):
                if self.board[new_row][new_col] == player:
                    # we found outflanking disc, append visited disc to flip list
                    flips += visited_discs
                    break
                elif self.board[new_row][new_col] == EMPTY:
                    # reached blank space, no discs could be outflanked
                    break
                else:
                    # add met disc to visited discs
                    visited_discs.append((new_row, new_col))
                new_row += dy
                new_col += dx

        return flips

    def has_valid_move(self, player) -> bool:
        for row in range(self.rows):
            for column in range(self.columns):
                valid, _ = self.validate_placing(row, column, player)
                if valid:
                    return True

        return False

    def valid_moves(self, player: int):
        moves = []
        for row in range(self.rows):
            for column in range(self.columns):
                valid, _ = self.validate_placing(row, column, player)
                if valid:
                    moves.append((row, column))

        return moves

    def get_winner(self) -> Union[int, None]:
        """
        Check if game ended and if so return winning player.
        If at least two players score the same, we call a draw.

        :return: winning player number or None
        """
        scores = [0, 0, 0]
        for row in self.board:
            for cell in row:
                scores[cell] += 1

        max_score = max(scores)

        if len(list(filter(lambda x: x == max_score, scores))) > 1:
            return None  # draw

        return scores.index(max_score)
