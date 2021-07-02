// from typing import List
//
// from board import Board
//
//
// class Heuristic:
//     def evaluate(self, board: Board, player: int) -> int:
//         raise NotImplementedError
//
//
// class GreedyHeuristic(Heuristic):
//     def evaluate(self, board: Board, player: int) -> int:
//         """
//         Move decision with greedy strategy based on board state.
//         """
//         return sum([sum(map(lambda cell: cell == player, row)) for row in board.board])
//
//
// class WeightedSumHeuristic(Heuristic):
//     weight_table = [
//         [60,  -30,  25,  25,  25,  25,  25, -30,  60],
//         [-30, -40, -25, -25, -25, -25, -25, -40, -30],
//         [25,  -25,   1,   1,   1,   1,   1, -25,  25],
//         [25,  -25,   1,   1,   1,   1,   1, -25,  25],
//         [25,  -25,   1,   1,   1,   1,   1, -25,  25],
//         [25,  -25,   1,   1,   1,   1,   1, -25,  25],
//         [25,  -25,   1,   1,   1,   1,   1, -25,  25],
//         [-30, -40, -25, -25, -25, -25, -25, -40, -30],
//         [60,  -30,  25,  25,  25,  25,  25, -30,  60],
//     ]
//
//     def evaluate(self, board: Board, player: int) -> int:
//         """
//         Move decision with weighted sum strategy based on board state.
//         """
//         return sum(
//             [
//                 sum(map(lambda x: (x[0] == player) * x[1], zip(row, row_weights)))
//                 for row, row_weights in zip(board.board, self.weight_table)
//             ]
//         )
//
//
// class WedgeHeuristic(Heuristic):
//     weight_table = [
//         [160, -30, 25, 25, 25, 25, 25, -30, 160],
//         [-30, -40, -25, -25, -25, -25, -25, -40, -30],
//         [25, -25, 1, 1, 1, 1, 1, -25, 25],
//         [25, -25, 1, 1, 1, 1, 1, -25, 25],
//         [25, -25, 1, 1, 1, 1, 1, -25, 25],
//         [25, -25, 1, 1, 1, 1, 1, -25, 25],
//         [25, -25, 1, 1, 1, 1, 1, -25, 25],
//         [-30, -40, -25, -25, -25, -25, -25, -40, -30],
//         [160, -30, 25, 25, 25, 25, 25, -30, 160],
//     ]
//
//     def evaluate(self, board: Board, player: int) -> int:
//         """
//         Move decision with wedge strategy based on board state.
//         """
//         self.weight_table = [
//             [160, -130, 25, 25, 25, 25, 25, -130, 160],
//             [-130, -40, -25, -25, -25, -25, -25, -40, -130],
//             [25, -25, 1, 1, 1, 1, 1, -25, 25],
//             [25, -25, 1, 1, 1, 1, 1, -25, 25],
//             [25, -25, 1, 1, 1, 1, 1, -25, 25],
//             [25, -25, 1, 1, 1, 1, 1, -25, 25],
//             [25, -25, 1, 1, 1, 1, 1, -25, 25],
//             [-130, -40, -25, -25, -25, -25, -25, -40, -130],
//             [160, -130, 25, 25, 25, 25, 25, -130, 160],
//         ]
//         if board.board[board.rows-1][board.columns-1] == -1: #right bottom corner is free
//             if board.board[board.rows-2][board.columns-1] > -1 and board.board[board.rows-2][board.columns-1] != player:
//                 if board.board[board.rows - 3][board.columns - 1] == -1:
//                   if board.board[board.rows-4][board.columns-1] > -1 and board.board[board.rows-4][board.columns-1] != player:
//                       self.weight_table[board.rows - 3][board.columns - 1] += 100
//             if board.board[board.rows - 1][board.columns - 2] > -1 and board.board[board.rows - 1][
//                 board.columns - 2] != player:
//                 if board.board[board.rows - 1][board.columns - 3] == -1:
//                     if board.board[board.rows - 1][board.columns - 4] > -1 and board.board[board.rows - 1][
//                         board.columns - 4] != player:
//                         self.weight_table[board.rows - 1][board.columns - 3] += 100
//
//         if board.board[board.rows-1][0] == -1: #left bottom corner is free
//             if board.board[board.rows-2][0] > -1 and board.board[board.rows-2][0] != player:
//                 if board.board[board.rows - 3][0] == -1:
//                   if board.board[board.rows-4][0] > -1 and board.board[board.rows-4][0] != player:
//                       self.weight_table[board.rows - 3][0] += 100
//             if board.board[board.rows - 1][1] > -1 and board.board[board.rows - 1][
//                 1] != player:
//                 if board.board[board.rows - 1][2] == -1:
//                     if board.board[board.rows - 1][3] > -1 and board.board[board.rows - 1][
//                         3] != player:
//                         self.weight_table[board.rows - 1][2] += 100
//
//         if board.board[0][0] == -1:  # left top corner is free
//             if board.board[1][0] > -1 and board.board[1][
//                 0] != player:
//                 if board.board[2][0] == -1:
//                     if board.board[3][0] > -1 and board.board[3][0] != player:
//                         self.weight_table[2][0] += 100
//             if board.board[0][1] > -1 and board.board[0][
//                 1] != player:
//                 if board.board[0][2] == -1:
//                     if board.board[0][3] > -1 and board.board[0][
//                         3] != player:
//                         self.weight_table[0][2] += 100
//
//         if board.board[0][board.columns - 1] == -1:  # right top corner is free
//             if board.board[1][board.columns - 1] > -1 and board.board[1][
//                 board.columns - 1] != player:
//                 if board.board[2][board.columns - 1] == -1:
//                     if board.board[3][board.columns - 1] > -1 and board.board[3][board.columns - 1] != player:
//                         self.weight_table[2][0] += 100
//             if board.board[0][board.columns - 2] > -1 and board.board[0][
//                 board.columns - 2] != player:
//                 if board.board[0][board.columns - 3] == -1:
//                     if board.board[0][board.columns - 4] > -1 and board.board[0][
//                         board.columns - 4] != player:
//                         self.weight_table[0][board.columns - 3] += 100
//
//         return sum(
//             [
//                 sum(map(lambda x: (x[0] == player) * x[1], zip(row, row_weights)))
//                 for row, row_weights in zip(board.board, self.weight_table)
//             ]
//         )
