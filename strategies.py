from board import Board


class Strategy:
    def get_move(self, board_: Board) -> (int, int):
        raise NotImplementedError


class GreedyStrategy(Strategy):
    def get_move(self, board_: Board) -> (int, int):
        """
        Move decision with greedy strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplementedError


class WeightedSumStrategy(Strategy):
    def get_move(self, board_: Board) -> (int, int):
        """
        Move decision with weighted sum strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplementedError


class WedgeStrategy(Strategy):
    def get_move(self, board_: Board) -> (int, int):
        """
        Move decision with wedge strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplementedError
