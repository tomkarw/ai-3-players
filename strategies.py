from board import Board


class Strategy:
    def get_move(self, board_: Board) -> (int, int):
        raise NotImplemented


class GreedyStrategy(Strategy):
    def get_move(self, board_: Board) -> (int, int):
        """
        Move decision with greedy strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplemented


class WeightedSumStrategy(Strategy):
    def get_move(self, board_: Board) -> (int, int):
        """
        Move decision with weighted sum strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplemented


class WedgeStrategy(Strategy):
    def get_move(self, board_: Board) -> (int, int):
        """
        Move decision with wedge strategy based on board state.
        """
        # TODO(tkarwowski)
        raise NotImplemented
