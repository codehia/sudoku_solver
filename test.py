import csv

import pytest

from main import solve


def parse_file(file_name):
    with open(file_name, "r") as test_file:
        dict_reader = csv.DictReader(test_file)
        for idx, row in enumerate(dict_reader):
            quiz = row["quizzes"]
            solution = row["solutions"]
            chunk_size = 9
            quiz_fmt = [
                list(quiz[i : i + chunk_size]) for i in range(0, len(quiz), chunk_size)
            ]
            solution_fmt = [
                list(solution[i : i + chunk_size])
                for i in range(0, len(solution), chunk_size)
            ]
            yield quiz_fmt, solution_fmt


@pytest.mark.parametrize("quiz, solution", parse_file("./test.csv"))
def test_solve(quiz, solution):
    assert solve(quiz) == solution, (
        f"Failed for quiz: {quiz} with expected solution: {solution}"
    )
