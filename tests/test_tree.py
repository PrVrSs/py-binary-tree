import pytest


fixtures_len = (
    ([1, 2, 3], 3),
    ([1, 1, 3], 2),
)

fixtures_get = (
    ([1, 2, 3], 3, 3),
    ([1, 2, 3], 4, None),
)


@pytest.mark.parametrize('test_data', fixtures_len)
def test_len_case(test_data, tree_creator):
    vector, expected = test_data

    tree = tree_creator(vector)
    assert len(tree) == expected


@pytest.mark.parametrize('test_data', fixtures_get)
def test_get_case(test_data, tree_creator):
    vector, get_val, expected = test_data
    tree = tree_creator(vector)

    assert tree.get(get_val) == expected


def test_repr(tree_creator):
    tree = tree_creator([10, 20, 6, 4])

    assert tree.__repr__() == 'BST(Node(10 -> left(Node(6 -> left(Node(4 -> left(None) right(None))) right(None))) right(Node(20 -> left(None) right(None)))))'
