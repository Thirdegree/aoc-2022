from contextlib import suppress
from enum import Enum
from functools import cmp_to_key
from itertools import zip_longest
from typing import Any


class CompareResult(Enum):
    left = -1
    right = 0
    same = 1


def compare(pack1: list[Any], pack2: list[Any]) -> CompareResult:
    for p1elem, p2elem in zip_longest(pack1, pack2):
        if p1elem is None:
            # exactly the same except pack1 is shorter
            return CompareResult.left
        if p2elem is None:
            # exactly the same except pack1 is longer
            return CompareResult.right
        if isinstance(p1elem, int) and isinstance(p2elem, int):
            if p1elem < p2elem:
                return CompareResult.left
            if p1elem > p2elem:
                return CompareResult.right
            continue
        # zero or one of these might be true, NOT both (never both)
        if isinstance(p1elem, int):
            p1elem = [p1elem]
        if isinstance(p2elem, int):
            p2elem = [p2elem]
        if (res := compare(p1elem, p2elem)) is not CompareResult.same:
            return res
    return CompareResult.same


def main():
    with open('src/data/day13.txt') as f:
        data = f.read().splitlines(keepends=False)
    data_iter = iter(data)
    all_packs = []
    for i, (packet1, packet2) in enumerate(zip(data_iter, data_iter)):
        all_packs.append(eval(packet1))  # we do a _minor_ crime
        all_packs.append(eval(packet2))  # ok two minor crimes
        with suppress(StopIteration):
            # discard the line breaks
            next(data_iter)
    first_break = [[2]]
    all_packs.append(first_break)
    second_break = [[6]]
    all_packs.append(second_break)
    all_packs = sorted(all_packs,
                       key=cmp_to_key(lambda a, b: compare(a, b).value))

    print(all_packs)
    print((all_packs.index(first_break) + 1) *
          (all_packs.index(second_break) + 1))


if __name__ == "__main__":
    main()
