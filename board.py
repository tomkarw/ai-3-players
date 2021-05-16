from dataclasses import dataclass

from colorama import Back, Style

EMPTY = -1

COLOR_MAPPING = {
    -1: "",
    0: Back.WHITE,
    1: Back.BLACK,
    2: Back.RED,
}


class ForbiddenMove(Exception):
    pass


@dataclass
class Board:
    """
    Stateful board of Othello game
    """
    _board: list[list[int]]
    rows: int = 9
    columns: int = 9

    def __init__(self) -> None:
        super().__init__()
        self._board = [[EMPTY for _ in range(self.columns)] for _ in range(self.rows)]

    def topbot_line(self) -> str:
        return "  " + Back.GREEN + " ".join(["+"] + (["-  "] * self.columns)[:-1] + [f"- +{Style.RESET_ALL}\n"])

    def __str__(self):
        output = ""
        # column numbering
        output += "    " + "   ".join(map(str, range(self.columns))) + "\n"
        # top line
        output += self.topbot_line()
        # iterate over all but last row
        for row_num, row in enumerate(self._board):
            output += f"{row_num} {Back.GREEN}|"
            for cell in row:
                output += self.print_piece(cell) + "|"
            output += f"{Style.RESET_ALL}\n"
            if row_num != self.rows - 1:
                output += "  " + Back.GREEN + " ".join([" "] + ["-  "] * self.columns) + Style.RESET_ALL + "\n"
        # bottom line
        output += self.topbot_line()
        return output

    @staticmethod
    def print_piece(cell):
        color = COLOR_MAPPING[cell]
        return f"{color or ''}   {Back.GREEN}"

    def setup_three_players(self):
        """
        Create starting position for 3 player game
        """
        self._board[3][3] = self._board[3][5] = self._board[4][4] = 0  # white
        self._board[3][4] = self._board[5][3] = self._board[5][5] = 1  # black
        self._board[4][3] = self._board[4][5] = self._board[5][4] = 2  # red

    def place(self, row: int, column: int, player: int):
        self.validate_placing(row, column, player)
        self._board[row][column] = player

    def validate_placing(self, row: int, column: int, player: int):
        # TODO(tkarwowski)
        # raise ForbiddenMove("human readable reason")
        pass

    def check_win(self):
        """
        Check if game ended and if so return winning player.

        :return: None or winning player number
        """
        # TODO(tkarwowski)
        return None
