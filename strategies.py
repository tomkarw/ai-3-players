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
    def evaluate(self, board: Board, player: int) -> int:
        """
        Move decision with weighted sum strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplementedError


class WedgeHeuristic(Heuristic):
    def evaluate(self, board: Board, player: int) -> int:
        """
        Move decision with wedge strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplementedError
