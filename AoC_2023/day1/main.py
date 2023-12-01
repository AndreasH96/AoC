import re

test_path1 = "./test_input1.txt"
test_path2 = "./test_input2.txt"
real_path = "./real_input.txt"

map_dict = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}


def solve_part1(file_path: str) -> int:
    with open(file=file_path) as f:
        nums = [re.sub("[^\d\.]", "", x) for x in f.readlines()]
        return sum([int("".join([x[0], x[-1]])) for x in nums])


def solve_part2(file_path: str) -> int:
    def numextract(text: str):
        matchers = list(map_dict.keys()) + [str(x) for x in range(1, 10)]
        res = []
        for matcher in matchers:
            positions = [m.start() for m in re.finditer(matcher, text)]
            for pos in positions:
                res.append((map_dict[matcher] if matcher in map_dict else matcher, pos))
        return [x[0] for x in sorted(res, key=lambda x: x[1])]

    with open(file=file_path) as f:
        lines = f.readlines()
        nums = [numextract(x) for x in lines]
        return sum([int("".join([x[0], x[-1]])) for x in nums])


if __name__ == "__main__":
    part1_test = solve_part1(test_path1)
    print(part1_test)
    part1_real = solve_part1(real_path)
    print(part1_real)

    part2_test = solve_part2(test_path2)
    print(part2_test)
    part2_real = solve_part2(real_path)
    print(part2_real)
