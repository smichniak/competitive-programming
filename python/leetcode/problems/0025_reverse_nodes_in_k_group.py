"""LeetCode 25. Reverse Nodes in k-Group.

https://leetcode.com/problems/reverse-nodes-in-k-group/
"""

PROBLEM_NUMBER = 25
PROBLEM_SLUG = "reverse-nodes-in-k-group"


class ListNode:
    def __init__(self, val=0, nxt=None):
        self.val = val
        self.next = nxt

    def __str__(self):
        if self.next is None:
            return str(self.val)
        return str(self.val) + " " + str(self.next)


def _list_to_nodes(values: list[int]) -> ListNode | None:
    head = None
    for v in reversed(values):
        head = ListNode(v, head)
    return head


def _nodes_to_list(head: ListNode | None) -> list[int]:
    out: list[int] = []
    while head is not None:
        out.append(head.val)
        head = head.next
    return out


class Solution:
    def reverseKGroup(self, head: ListNode, k: int) -> ListNode:
        n = 0
        current = head
        while current is not None:
            current = current.next
            n += 1
        trap = ListNode()
        trap.next = head

        def reverse(firsNotToBeReversed: ListNode, k: int) -> ListNode:
            first = firsNotToBeReversed.next
            current = first.next
            before = first

            for _ in range(k - 1):
                tmp = current.next
                current.next = before
                before = current
                current = tmp

            firsNotToBeReversed.next = before
            first.next = current
            return first

        trap1 = trap
        for _ in range(n // k):
            trap1 = reverse(trap1, k)

        return trap.next


def test_example_from_statement() -> None:
    sol = Solution()
    head = _list_to_nodes([1, 2, 3, 4, 5])
    assert _nodes_to_list(sol.reverseKGroup(head, 3)) == [3, 2, 1, 4, 5]


def test_k_equals_one() -> None:
    sol = Solution()
    head = _list_to_nodes([1, 2, 3])
    assert _nodes_to_list(sol.reverseKGroup(head, 1)) == [1, 2, 3]


def test_full_length_multiple_of_k() -> None:
    sol = Solution()
    head = _list_to_nodes([1, 2, 3, 4])
    assert _nodes_to_list(sol.reverseKGroup(head, 2)) == [2, 1, 4, 3]
