def get_content():
    with open("key.txt", "r") as file:
        return file.read()


content = get_content()
content_array = content.split("\n")
LEN = len(content_array)


def split_index(array: list, index: int):
    a1 = []
    a2 = []
    for count, value in enumerate(array):
        if count == index:
            continue
        elif count > index:
            a2.append(value)
        else:
            a1.append(value)
    return a1, a2


def split_vertical(x: int, y: int) -> tuple[list[int], list[int]]:
    vertical_slice: list[int] = []
    for i in range(LEN):
        vertical_slice.append(int(content_array[i][x]))

    return split_index(vertical_slice, y)


def split_horozontal(x: int, y: int) -> tuple[list[int], list[int]]:
    horizontal_slice: list[int] = []
    for i in content_array[y]:
        horizontal_slice.append(int(i))
    return split_index(horizontal_slice, x)


def get_visible(y: int, x: int):
    if y == 0 or x == 0 or y == LEN or x == LEN:
        return True
    value = int(content_array[y][x])
    u, d = split_vertical(x, y)
    l, r = split_horozontal(x, y)
    direction_values: list[list[int]] = [u, d, l, r]  # up, down, left, right

    direction_visible_count = 4
    for direction in direction_values:
        for tree in direction:
            if tree >= value:
                direction_visible_count -= 1
                break
    return direction_visible_count > 0


def main():
    count = 0
    for num_i in range(len(content_array)):
        for let_i in range(len(content_array[num_i])):
            if get_visible(num_i, let_i):
                count += 1

    print(count)


main()
