from py_binary_tree import Tree


def test_len_case():
    tree = Tree()

    tree.insert(1)
    tree.insert(2)
    tree.insert(3)
    assert len(tree) == 3


def test_len_case_2():
    tree = Tree()

    tree.insert(1)
    tree.insert(1)
    tree.insert(3)
    assert len(tree) == 2


def test_get_case():
    tree = Tree()

    tree.insert(1)
    tree.insert(2)
    tree.insert(3)
    assert tree.get(3) == 3


def test_get_case2():
    tree = Tree()

    tree.insert(1)
    tree.insert(2)
    tree.insert(3)
    assert tree.get(4) is None
