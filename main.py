from pprint import pp
import copy


# TODO: Consider this approach
# {'1': (({R}, ...), ({C}, ...), ({B}, ...)), '2': {}, ...}

"""
[['2', '1', '8', '9', '5', '4', '7', '6', '3'],
 ['7', '6', '3', '1', '2', '8', '5', '9', '4'],
 ['5', '9', '4', '6', '7', '3', '2', '1', '8'],
 ['4', '2', '9', '5', '3', '6', '8', '7', '1'],
 ['3', '5', '6', '7', '8', '1', '0', '2', '9'],
 ['8', '7', '1', '2', '4', '9', '3', '5', '6'],
 ['9', '8', '2', '4', '6', '5', '1', '3', '7'],
 ['1', '3', '7', '8', '9', '2', '6', '4', '5'],
 ['6', '4', '5', '3', '1', '7', '9', '8', '2']]
"""

"""
ELEM: (i, j)
ELEM: -> R, C, B

while:
    ({0..9} | candidates for (i, j)) - R - C - B
    candidates -> len(candidates) == 1 -> final_value
"""
type Puzzle = list[list[str]]
type ElementIndex = tuple[int, int]


CANDIDATES = {
    # (2, 3): {1, 2, 4}
}

EMPTY_VALUE_PLACEHOLDER = '0'
BLOCK_SIZE = 3




def get_column_values(element_index: ElementIndex, puzzle: list[list[str]]) -> set[str]:
    # TODO: Consider pivot of the puzzle
    column_values = set()
    for row in puzzle:
        for col_idx, val in enumerate(row):
            if (col_idx == element_index[1]) and (val != EMPTY_VALUE_PLACEHOLDER):
                column_values.add(val)
    return column_values


def get_row_values(element_index: ElementIndex, puzzle: list[list[str]]) -> set[str]:
    return {elem for elem in puzzle[element_index[0]] if elem != EMPTY_VALUE_PLACEHOLDER}


def get_block_values(element_index: ElementIndex, puzzle: list[list[str]]) -> set[str]:
    row_start = element_index[0] // 3 * 3
    col_start = element_index[1] // 3 * 3
    return {
        puzzle[row_idx][col_idx]
        for row_idx in range(row_start, row_start + 3)
        for col_idx in range(col_start, col_start + 3)
        if puzzle[row_idx][col_idx] != EMPTY_VALUE_PLACEHOLDER
    }

def solve(puzzle: list[list[str]]) -> list[list[str]]:
    # Each list inside the outer list is a ROW (R)
    # Column (C)
    # Block (B)

    # for each [i][j] of the puzzle ->
    # get_column_values
    # get_row_values
    # get_block_values
    solution = copy.deepcopy(puzzle)

    # TODO: Replace
    return [[str(i) for i in range(1, 10)] for _ in range(9)]

def main():
    ds = [['2', '1', '8', '9', '5', '4', '7', '6', '3'],
          ['7', '6', '3', '1', '2', '8', '5', '9', '4'],
          ['5', '9', '4', '6', '7', '3', '2', '1', '8'],
          ['4', '2', '9', '5', '3', '6', '8', '7', '1'],
          ['3', '5', '6', '7', '8', '1', '0', '2', '9'],
          ['8', '7', '1', '2', '4', '9', '3', '5', '6'],
          ['9', '8', '2', '4', '6', '5', '1', '3', '7'],
          ['1', '3', '7', '8', '9', '2', '6', '4', '5'],
          ['6', '4', '5', '3', '1', '7', '9', '8', '2']]
    solution = solve(ds)
    pp(solution)

if __name__ == "__main__":
    main()
