from dataclasses import dataclass

from colorama import Back, Style

EMPTY = -1


class Color:
    WHITE = r"\e[47m"
    BLACK = r"\e[40m"
    RED = r"\e[41m"
    RESET = r"\033[0m"

    list = [WHITE, BLACK, RED]


@dataclass
class Board:
    """
    Stateful board of othello game
    """
    _board: list[list[int]]
    rows: int = 9
    columns: int = 9

    def __init__(self) -> None:
        super().__init__()
        self._board = [[EMPTY for _ in range(self.columns)] for _ in range(self.rows)]

    def __str__(self):
        # top line
        output = Back.GREEN + " ".join(["+"] + (["-  "] * self.columns)[:-1] + [f"- +{Style.RESET_ALL}\n"])
        # iterate over all but last row
        for row_num, row in enumerate(self._board):
            output += f"{Back.GREEN}|"
            for cell in row:
                output += self.print_piece(cell) + "|"
            output += f"{Style.RESET_ALL}\n"
            if row_num != self.rows - 1:
                output += Back.GREEN + " ".join([" "] + ["-  "] * self.columns) + Style.RESET_ALL + "\n"
        # bottom line
        output += Back.GREEN + " ".join(["+"] + (["-  "] * self.columns)[:-1] + [f"- +{Style.RESET_ALL}\n"])
        return output

    @staticmethod
    def print_piece(cell):
        color = None
        if cell == 0:
            color = Back.WHITE
        elif cell == 1:
            color = Back.BLACK
        elif cell == 2:
            color = Back.RED
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

    def validate_placing(self, row: int, column: int, player: int):
        pass


if __name__ == "__main__":
    board = Board()
    board.setup_three_players()
    print(board)
