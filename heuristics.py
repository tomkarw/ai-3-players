from board import Board


class Heuristic:
    def evaluate(self, board: Board, player: int) -> int:
        raise NotImplementedError


class GreedyHeuristic(Heuristic):
    def evaluate(self, board: Board, player: int) -> int:
        """
        Move decision with greedy strategy based on board state.
        """
        return sum([sum(map(lambda cell: cell == player, row)) for row in board.board])


class WeightedSumHeuristic(Heuristic):
    weight_table = [
        [6, 5, 3, 3, 3, 3, 3, 5, 6],
        [5, 4, 2, 2, 2, 2, 2, 4, 5],
        [3, 2, 1, 1, 1, 1, 1, 2, 3],
        [3, 2, 1, 1, 1, 1, 1, 2, 3],
        [3, 2, 1, 1, 1, 1, 1, 2, 3],
        [3, 2, 1, 1, 1, 1, 1, 2, 3],
        [3, 2, 1, 1, 1, 1, 1, 2, 3],
        [5, 4, 2, 2, 2, 2, 2, 4, 5],
        [6, 5, 3, 3, 3, 3, 3, 5, 6],
    ]

    def evaluate(self, board: Board, player: int) -> int:
        """
        Move decision with weighted sum strategy based on board state.
        """
        return sum(
            [
                sum(map(lambda x: (x[0] == player) * x[1], zip(row, row_weights)))
                for row, row_weights in zip(board.board, self.weight_table)
            ]
        )


class WedgeHeuristic(Heuristic):
    def evaluate(self, board: Board, player: int) -> int:
        """
        Move decision with wedge strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplementedError
