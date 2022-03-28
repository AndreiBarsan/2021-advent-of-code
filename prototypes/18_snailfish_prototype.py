from __future__ import annotations
from dataclasses import dataclass
from typing import Optional


import copy
import math
from tqdm import tqdm


def add(s1, s2):
    new_parent = Node(parent=None, value=None, left=s1, right=s2)
    s1.parent = new_parent
    s2.parent = new_parent
    return new_parent


def eq(s1, s2):
    if s1.value is not None:
        if s2.value is not None:
            return s1.value == s2.value
        else:
            return False
    elif s2.value is not None:
        return False

    eq_left = eq(s1.left, s2.left)
    eq_right = eq(s1.right, s2.right)

    return eq_left and eq_right


def first_explode(s, depth=0):
    new_depth = depth + 1

    if new_depth >= 4:
        if s.left.value is None:
            return s.left
        elif s.right.value is None:
            return s.right
        else:
            return None
    else:
        if s.left.value is None:
            left_cand = first_explode(s.left, new_depth)
            if left_cand is not None:
                return left_cand
        if s.right.value is None:
            return first_explode(s.right, new_depth)
        else:
            return None



node_id = 1

@dataclass
class Node:
    parent: Optional[Node]
    value: Optional[Node]
    left: Optional[Node]
    right: Optional[Node]
    _id: int = -1

    def __eq__(self, other: object) -> bool:
        return self._id == other._id

    def __repr__(self) -> bool:
        if self.value is not None:
            return str(self.value)
        else:
            return "[" + str(self.left) + ", " + str(self.right) + "]"

    def magnitude(self) -> bool:
        if self.value is not None:
            return self.value

        return 3 * self.left.magnitude() + 2 * self.right.magnitude()

    def __post_init__(self):
        global node_id
        self._id = node_id
        node_id = node_id + 1



def find_first_right_parent(node: Node) -> Optional[Node]:
    if node.parent is None:
        return None
    elif node.parent.right == node:
        return node.parent
    else:
        return find_first_right_parent(node.parent)


def find_first_left_parent(node: Node) -> Optional[Node]:
    if node.parent is None:
        return None
    elif node.parent.left == node:
        return node.parent
    else:
        return find_first_left_parent(node.parent)


def find_rightmost_child(node: Node):
    if node.right is None:
        return node
    else:
        return find_rightmost_child(node.right)


def find_leftmost_child(node: Node):
    if node.left is None:
        return node
    else:
        return find_leftmost_child(node.left)



def find_immediate_left_sibling(node: Node) -> Optional[Node]:
    # assert called on a leaf pair
    # if node.left.value is None or node.right.value is None:
    #     raise RuntimeError("Called sibling finding on non-leaf! {}".format(node))
    par = find_first_right_parent(node)
    if par is None:
        return None
    return find_rightmost_child(par.left)

def find_immediate_right_sibling(node: Node) -> Optional[Node]:
    # assert called on a leaf pair
    # if node.left.value is None or node.right.value is None:
    #     raise RuntimeError("Called sibling finding on non-leaf! {}".format(node))
    par = find_first_left_parent(node)
    if par is None:
        return None
    return find_leftmost_child(par.right)

def as_nodes(list_repr, parent=None):
    current = Node(parent, value=None, left=None, right=None)
    if isinstance(list_repr[0], list):
        current.left = as_nodes(list_repr[0], current)
    else:
        current.left = Node(current, value=list_repr[0], left=None, right=None)

    if isinstance(list_repr[1], list):
        current.right = as_nodes(list_repr[1], current)
    else:
        current.right = Node(current, value=list_repr[1], left=None, right=None)

    return current


def as_list(node_repr):
    if node_repr.left is None and node_repr.right is None:
        assert node_repr.value is not None
        return node_repr.value
    else:
        l = as_list(node_repr.left)
        r = as_list(node_repr.right)
        return [l, r]


def explode_in_place(snail_nr) -> bool:
    node_to_explode = first_explode(snail_nr)
    # print("First explode:", node_to_explode)
    if node_to_explode is not None:
        ls = find_immediate_left_sibling(node_to_explode)
        rs = find_immediate_right_sibling(node_to_explode)
        # print(ls, rs)

        if ls is not None:
            ls.value += node_to_explode.left.value

        if rs is not None:
            rs.value += node_to_explode.right.value

        if node_to_explode.parent.left == node_to_explode:
            # Replace left node
            node_to_explode.parent.left = Node(
                node_to_explode.parent,
                value=0,
                left=None,
                right=None
            )
        elif node_to_explode.parent.right == node_to_explode:
            # Replace right node
            node_to_explode.parent.right = Node(
                node_to_explode.parent,
                value=0,
                left=None,
                right=None
            )
        else:
            raise ValueError()

        return True
    return False


def split_in_place(snail_nr):
    node_to_split = first_split_candidate(snail_nr)
    # print("Check for split for:", snail_nr)
    if node_to_split is not None:
        node_to_split, val = node_to_split
        # print("Splitting:")
        # print(node_to_split)
        lv = int(math.floor(val / 2.0))
        rv = int(math.ceil(val / 2.0))
        if val == node_to_split.left.value:
            node_to_split.left = Node(node_to_split, value=None, left=None, right=None)
            node_to_split.left.left = Node(node_to_split.left, value=lv, left=None, right=None)
            node_to_split.left.right = Node(node_to_split.left, value=rv, left=None, right=None)
        elif val == node_to_split.right.value:
            node_to_split.right = Node(node_to_split, value=None, left=None, right=None)
            node_to_split.right.left = Node(node_to_split.right, value=lv, left=None, right=None)
            node_to_split.right.right = Node(node_to_split.right, value=rv, left=None, right=None)
        else:
            raise ValueError()

        return True

    return False


def first_split_candidate(s):
    """DFS for the leftmost node >= 10."""
    if s.left is not None:
        if s.left.value is not None and s.left.value >= 10:
            return s, s.left.value

        left_candidate = first_split_candidate(s.left)
        if left_candidate is not None:
            return left_candidate

    if s.right is not None:
        if s.right.value is not None and s.right.value >= 10:
            return s, s.right.value

        right_candidate = first_split_candidate(s.right)
        if right_candidate is not None:
            return right_candidate

    return None


def reduce(snail_nr):
    """Returns a fully reduced version of the given snail number."""
    new_snail_nr = copy.deepcopy(snail_nr)
    # print("Start:")
    # print(snail_nr)

    while True:
        # print("\nNew reduction phase...")
        if explode_in_place(new_snail_nr):
            # print("Exploded:", new_snail_nr)
            continue
        # else:
        #     print("No explode.")

        if split_in_place(new_snail_nr):
            # print("Split:", new_snail_nr)
            continue
        # else:
        #     print("No split.")

        break

    # print(new_snail_nr)
    return new_snail_nr


def check_reduce(original, expected_reduced):
    if isinstance(original, list):
        original = as_nodes(original)
    if isinstance(expected_reduced, list):
        expected_reduced = as_nodes(expected_reduced)

    actual_reduced = reduce(original)
    if not eq(expected_reduced, actual_reduced):
        print("Comparison failed:")
        print("Original:", original)
        print("+---------------------------------+")
        print("Expected:", expected_reduced)
        print("Actual:  ", actual_reduced)
        print("+---------------------------------+")
        raise ValueError()



def test_find_first_right_parent():
    e1 = [[[[[9,8],1],2],3],4]
    e1_n = as_nodes(e1)
    assert e1 == as_list(e1_n)

    e2 = [[[9,[5,2]],[[5,2],[6,8]]],[[[7,0],7],[[2,3],[9,4]]]]
    e2_n = as_nodes(e2)
    assert e2 == as_list(e2_n)

    assert first_explode(as_nodes([[[[0,7],4],[7,[[8,4],9]]],[1,1]])).left.value == 8
    assert first_explode(as_nodes([[[[0,7],4],[7,[[8,4],9]]],[1,1]])).right.value == 4

    assert find_first_right_parent(e2_n.right.left.left) == e2_n
    assert find_rightmost_child(e2_n) == e2_n.right.right.right.right
    assert find_rightmost_child(e1_n).value == 4

    assert find_immediate_left_sibling(e1_n.right).value == 3
    assert find_immediate_left_sibling(e1_n.left.left.right).value == 1

    assert find_immediate_right_sibling(e1_n.right) == None
    assert find_immediate_right_sibling(e1_n.left.left.right).value == 3

    real_sample_a = as_nodes(
        [
            [[0, 6], [8, [8, 8]]],
            [[0, 15], [21, 0]]
        ],
        # [[2, [11, 6]], [[0, 12], [8, 0]]]
    )
    pair_of_8 = real_sample_a.left.right.right
    # print(pair_of_8)
    # print(find_immediate_right_sibling(pair_of_8))
    assert find_immediate_right_sibling(real_sample_a.left.right.right).value == 0

    assert first_split_candidate(as_nodes([10, 1]))[0].left.value == 10
    assert first_split_candidate(as_nodes([10, 10]))[0].left.value == 10
    assert first_split_candidate(as_nodes([1, 10]))[0].right.value == 10
    assert first_split_candidate(as_nodes([[1, 11], 10]))[0].right.value == 11
    assert first_split_candidate(as_nodes([[1, 3], [10, [64, 1]]]))[0].left.value == 10


    check_reduce([10, 1], [[5, 5], 1])
    check_reduce([11, 1], [[5, 6], 1])
    check_reduce([12, 1], [[6, 6], 1])
    check_reduce([1, 10], [1, [5, 5]])
    check_reduce([1, 11], [1, [5, 6]])
    check_reduce([1, [1, 11]], [1, [1, [5, 6]]])


    check_reduce(
      [[[[1, 2], 2], [[[4, 7], 0], [1, 2]], 4], 1],
      [[[[1, 2], 6], [[0, 7], [1, 2]], 4], 1])

    check_reduce(
        [7,[6,[5,[4,[3,2]]]]],
        [7,[6,[5,[7,0]]]]
    )
    check_reduce(
        [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]],
        [[3,[2,[8,0]]],[9,[5,[7,0]]]]
    )
    check_reduce(
        [[[[[4,3],4],4],[7,[[8,4],9]]], [1,1]],
        [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
    )
    check_reduce(
        [15, 2],
        [[7, 8], 2],
    )
    check_reduce(
        [[[15, 3], 2], 1],
        [[[[7, 8], 3], 2], 1],
    )
    check_reduce(
        [[[[15, 3], 2], 1], 0],
        [[[[5, 0], 8], 1], 0],
    )
    check_reduce(
        [0, [[[15, 3], 2], 1]],
        [7, [[[5, 0], 8], 1]]
    )
    check_reduce(
        [[0, 0], [[[15, 3], 2], 1]],
        [[0, 7], [[[5, 0], 8], 1]]
    )
    # check_reduce(
    #     [[[[0, 6], [8, [8, 8]]], [[0, 15], [21, 0]]], [[2, [11, 6]], [[0, 12], [8, 0]]]]
    # )

    print("Micro-test OK.")





def snailfish():
    # input_fpath = "input/18-demo-03.txt"
    input_fpath = "input/18.txt"
    with open(input_fpath, "rt") as f:
        els = [as_nodes(eval(l)) for l in f.readlines()]

    # print(els)
    test_find_first_right_parent()

    # e1 = as_nodes([[[[[9,8],1],2],3],4])
    # r1 = reduce(e1)

    # e2 = as_nodes([7,[6,[5,[4,[3,2]]]]])
    # r2 = reduce(e2)

    # e3 = as_nodes([[6,[5,[4,[3,2]]]],1])
    # r3 = reduce(e3)

    # e4 = as_nodes([[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]])
    # r4 = reduce(e4)

    red_cand = add(as_nodes([[[[4,3],4],4],[7,[[8,4],9]]]), as_nodes([1, 1]))
    # print(red_cand)
    reduced = reduce(red_cand)
    # print(reduced)
    # print([[[[0,7],4],[[7,8],[6,0]]],[8,1]])
    # assert eq(reduced, as_nodes([[[[0,7],4],[[7,8],[6,0]]],[8,1]]))


    # print("Starting big example...")
    # res_big = reduce(add(
    #         as_nodes(
    #             [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
    #         ),
    #         as_nodes(
    #             [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    #         )
    #     ))
    # exp = as_nodes([[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]])
    # print()
    # print(exp)
    # print(res_big)
    # assert eq(res_big,exp)

    # wat = as_nodes([[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]])
    # print(reduce(wat))
    # print(wat)

    # ex

    a = as_nodes([[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]])
    b = as_nodes([7,[[[3,7],[4,3]],[[6,3],[8,8]]]])
    ss = add(a, b)
    print("\n\nStarting semi-big reduction")
    red_ss = reduce(ss)
    print()
    print("Orig:   ", ss)
    print("Reduced:", red_ss)
    print("Correct:", [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]])

    c = as_nodes([[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]])
    d = as_nodes([[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]])
    ss = add(c, d)
    red_ss = reduce(ss)
    print("Orig:   ", ss)
    print("Reduced:", red_ss)
    print("Correct:", [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]])

    # print(f"{red_ss.magnitude()}")

    # sample = as_nodes([[1,2],[[3,4],5]])
    # print(f"{sample.magnitude()}")

    assert as_nodes([[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]).magnitude() == 3488

    cur = copy.deepcopy(els[0])
    for el in els[1:]:
        res = add(cur, copy.deepcopy(el))
        res= reduce(res)
        cur = res

    max_mag = -1
    cands = [copy.deepcopy((e1, e2)) for e1 in els for e2 in els]
    for (e1, e2) in tqdm(cands):
        if e1 == e2:
            continue

        mag = reduce(add(e1, e2)).magnitude()
        if mag > max_mag:
            max_mag = mag


    print(cur)
    print(cur.magnitude())
    print(f"Max magnitude: {max_mag}")


if __name__ == "__main__":
    snailfish()