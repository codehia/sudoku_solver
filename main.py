from pprint import pp
import copy
import itertools

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

# TODO: Consider this approach
# {'1': (({R}, ...), ({C}, ...), ({B}, ...)), '2': {}, ...}
"""
ELEM: (i, j)
ELEM: -> R, C, B

while:
    ({0..9} | candidates for (i, j)) - R - C - B
    candidates -> len(candidates) == 1 -> final_value
"""
type PuzzleType = list[list[str]]
type ElementIndexType = tuple[int, int]

CANDIDATES = {} # (2, 3): {1, 2, 4}
EMPTY_VALUE_PLACEHOLDER = '0'
BLOCK_SIZE = 3

def get_column_values(element_index: ElementIndexType, puzzle: PuzzleType) -> set[str]:
    # TODO: Consider pivot of the puzzle
    column_values = set()
    for row in puzzle:
        for col_idx, val in enumerate(row):
            if (col_idx == element_index[1]) and (val != EMPTY_VALUE_PLACEHOLDER):
                column_values.add(val)
    return column_values

def get_row_values(element_index: ElementIndexType, puzzle: PuzzleType) -> set[str]:
    return {elem for elem in puzzle[element_index[0]] if elem != EMPTY_VALUE_PLACEHOLDER}

def get_block_values(element_index: ElementIndexType, puzzle: PuzzleType) -> set[str]:
    row_start = element_index[0] // 3 * 3
    col_start = element_index[1] // 3 * 3
    return {
        puzzle[row_idx][col_idx]
        for row_idx in range(row_start, row_start + 3)
        for col_idx in range(col_start, col_start + 3)
        if puzzle[row_idx][col_idx] != EMPTY_VALUE_PLACEHOLDER
    }

def is_solved(puzzle: PuzzleType) -> bool:
    return EMPTY_VALUE_PLACEHOLDER not in set(itertools.chain.from_iterable(puzzle))

def populate_candidates(puzzle: PuzzleType):
    for row_idx, row in enumerate(puzzle):
        for col_idx, value in enumerate(row):
            if value == EMPTY_VALUE_PLACEHOLDER:
                element_index = (row_idx, col_idx)
                column_values = get_column_values(element_index, puzzle)
                row_values = get_row_values(element_index, puzzle)
                block_values = get_block_values(element_index, puzzle)
                CANDIDATES[element_index] = set(map(str, range(1, 10))) - (column_values | row_values | block_values)

def solve(puzzle: PuzzleType) -> PuzzleType:
    populate_candidates(puzzle)
    solution = copy.deepcopy(puzzle)

    while not is_solved(solution):
        prev_candidates = copy.deepcopy(CANDIDATES)

        for row_idx, row in enumerate(solution):
            for col_idx, col in enumerate(row):
                if (candidates := CANDIDATES.get((row_idx, col_idx))) is None:
                    continue
                if len(candidates) == 1:
                    solution[row_idx][col_idx] = candidates.pop()
                    CANDIDATES.pop((row_idx, col_idx), None)
                # Remove the value from candidates
                elif len(candidates) > 1:
                    column_values = get_column_values((row_idx, col_idx), solution)
                    row_values = get_row_values((row_idx, col_idx), solution)
                    block_values = get_block_values((row_idx, col_idx), solution)
                    new_candidates = candidates  - (column_values | row_values | block_values)
                    CANDIDATES[(row_idx, col_idx)] = new_candidates
        if prev_candidates == CANDIDATES:
            print("No changes made in this iteration, puzzle might be unsolvable or requires a different approach.")
            return solution  # Return the current state of the puzzle if no changes were made
    return solution

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
