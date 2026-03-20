#!/usr/bin/env python3

import os


def main():
    for puzzle_path in filter(lambda f: f.endswith(".txt"), os.listdir(".")):
        new_lines = None
        with open(puzzle_path, "r") as file:
            new_lines = map(lambda line: line.split()[1] if len(line.split()) > 1 else line, file.readlines())

        with open(puzzle_path, "w") as file:
            file.write("\n".join(new_lines))


if __name__ == "__main__":
    main()