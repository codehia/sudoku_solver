import csv

import pytest

from main import get_block_values, get_column_values, get_row_values, solve


def parse_file(file_name):
    with open(file_name, "r") as test_file:
        dict_reader = csv.DictReader(test_file)
        for idx, row in enumerate(dict_reader):
            quiz = row["quizzes"]
            solution = row["solutions"]
            chunk_size = 9
            quiz_fmt = [
                list(quiz[i: i + chunk_size]) for i in range(0, len(quiz), chunk_size)
            ]
            solution_fmt = [
                list(solution[i: i + chunk_size])
                for i in range(0, len(solution), chunk_size)
            ]
            yield quiz_fmt, solution_fmt


@pytest.mark.parametrize("quiz, solution", parse_file("./test.csv"))
def test_solve(quiz, solution):
    assert solve(quiz) == solution, (
        f"Failed for quiz: {quiz} with expected solution: {solution}"
    )


def test_get_column_values():
    puzzle = [['2', '1', '8', '9', '5', '4', '7', '6', '3'],
              ['7', '6', '3', '1', '2', '8', '5', '9', '4'],
              ['5', '9', '4', '6', '7', '3', '2', '1', '8'],
              ['4', '2', '9', '5', '3', '6', '8', '7', '1'],
              ['3', '5', '6', '7', '8', '1', '0', '2', '9'],
              ['8', '7', '1', '2', '4', '9', '3', '5', '6'],
              ['9', '8', '2', '4', '6', '5', '1', '3', '7'],
              ['1', '3', '7', '8', '9', '2', '6', '4', '5'],
              ['6', '4', '5', '3', '1', '7', '9', '8', '2']]
    idx_column_mapping = {
        (0, 0): {'2', '7', '5', '4', '3', '8', '9', '1', '6'}
    }
    for idx, expected_col_values in idx_column_mapping.items():
        column_values = get_column_values(idx, puzzle)
        assert column_values == expected_col_values


def test_get_row_values():
    puzzle = [['2', '1', '8', '9', '5', '4', '7', '6', '3'],
              ['7', '6', '3', '1', '2', '8', '5', '9', '4'],
              ['5', '9', '4', '6', '7', '3', '2', '1', '8'],
              ['4', '2', '9', '5', '3', '6', '8', '7', '1'],
              ['3', '5', '6', '7', '8', '1', '0', '2', '9'],
              ['8', '7', '1', '2', '4', '9', '3', '5', '6'],
              ['9', '8', '2', '4', '6', '5', '1', '3', '7'],
              ['1', '3', '7', '8', '9', '2', '6', '4', '5'],
              ['6', '4', '5', '3', '1', '7', '9', '8', '2']]
    idx_column_mapping = {
        (0, 0): {'2', '1', '8', '9', '5', '4', '7', '6', '3'}
    }
    for idx, expected_col_values in idx_column_mapping.items():
        column_values = get_row_values(idx, puzzle)
        assert column_values == expected_col_values


def test_get_block_values():
    puzzle = [['2', '1', '8', '9', '5', '4', '7', '6', '3'],
              ['7', '6', '3', '1', '2', '8', '5', '9', '4'],
              ['5', '9', '4', '6', '7', '3', '2', '1', '8'],
              ['4', '2', '9', '5', '3', '6', '8', '7', '1'],
              ['3', '5', '6', '7', '8', '1', '0', '2', '9'],
              ['8', '7', '1', '2', '4', '9', '3', '5', '6'],
              ['9', '8', '2', '4', '6', '5', '1', '3', '7'],
              ['1', '3', '7', '8', '9', '2', '6', '4', '5'],
              ['6', '4', '5', '3', '1', '7', '9', '8', '2']]
    idx_column_mapping = {
        (7, 6): {'1', '3', '7', '6', '4', '5', '9', '8', '2'}
    }

    for idx, expected_col_values in idx_column_mapping.items():
        column_values = get_block_values(idx, puzzle)
        assert column_values == expected_col_values
