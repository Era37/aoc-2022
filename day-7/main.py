def read_file():
    with open("input.txt", "r") as f:
        return f.read()


def edit_dict(map: dict, dir: list, key: str, value: any) -> dict:
    local_map = map
    for folder in dir:
        local_map = local_map[folder]
    local_map[key] = value
    return map


def directory_index(val1: str) -> int | dict:
    returnValue: dict | int
    if val1 == "dir":
        returnValue = {}
    else:
        returnValue = int(val1)
    return (returnValue)


def evalutate_folder(folder: dict, total: list) -> int:
    t = 0
    for value in folder.values():
        if type(value) is dict:
            t += evalutate_folder(value, total)
        else:
            t += value
    total.append(t)
    return t


def add_up_values(folder: dict) -> list:
    folders = []
    for value in folder.values():
        if type(value) is dict:
            folders.extend(add_up_values(value))
        else:
            folders.append(value)
    return folders


def part_2(values: list, total: int) -> int:
    STORAGE = 70_000_000
    fuck_me: int = float("inf")
    for val in values:
        if (STORAGE - (total - val)) > 30_000_000 and val < fuck_me:
            fuck_me = val
    return fuck_me


def main():
    content: list = read_file().split("\n")
    dir: list = []
    map: dict = {}
    for line in content:
        if line.startswith("$ cd"):
            clean_line = line.replace("$ cd", "").strip()
            if clean_line == "/":
                dir = []
            elif clean_line == "..":
                del dir[-1]
            else:
                dir.append(clean_line)
        elif line.startswith("$ ls"):
            continue
        else:
            line_values = line.split(" ")
            val1, val2 = line_values
            value = directory_index(val1)
            if len(dir) == 0:
                map[val2] = value
            else:
                map = edit_dict(map, dir, val2, value)
    total_entries = []
    total = 0
    evalutate_folder(map, total_entries)
    for i in total_entries:
        if i <= 100_000:
            total += i
    print(part_2(total_entries, sum(add_up_values(map))), sum(add_up_values(map)))


if __name__ == "__main__":
    main()
