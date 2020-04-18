import pytest

from py_binary_tree import Tree


@pytest.fixture
def tree_creator():
    tree = Tree()

    def inner(elements):
        for element in elements:
            tree.insert(element)

        return tree

    return inner
