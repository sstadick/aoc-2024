def numbers():

    with open("../../day-01/input1_bigger.txt", "r") as fh:
        for line in fh:
            parts = line.split()
            yield (int(parts[0]), int(parts[1]))


def main():
    lhs = list()
    rhs = list()
    with open("../../day-01/input1_bigger.txt", "r") as fh:
        for line in fh:
            parts = line.split()
            lhs.append(int(parts[0]))
            rhs.append(int(parts[1]))

    lhs.sort()
    rhs.sort()
    diff = sum(abs(l - r) for l, r in zip(lhs, rhs))
    print(diff)


def main_ja():
    group_1 = list()
    group_2 = list()
    with open("../../day-01/input1_bigger.txt", "r") as csv_file:
        # csv_reader = csv.reader(csv_file, delimiter=" ")
        for row in csv_file:
            parts = row.split()
            group_1.append(int(parts[0]))
            group_2.append(int(parts[1]))

    group_1.sort()
    group_2.sort()
    difference = sum([abs(g1 - g2) for g1, g2 in zip(group_1, group_2)])
    print(difference)


if __name__ == "__main__":
    main()
