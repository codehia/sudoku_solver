import csv

def parse_file(file_name):
    with open(file_name, 'r') as test_file:
       dict_reader = csv.DictReader(test_file)
       for idx, row in enumerate(dict_reader):
           my_string = row['quizzes']
           chunk_size = 9
           temp = [list(a) for a in [my_string[i:i + chunk_size] for i in range(0, len(my_string), chunk_size)]]
           from pprint import pp
           pp(temp)

def main():
    print("Hello from sudoku-solver!")
    parse_file('./testcases.csv')


if __name__ == "__main__":
    main()
