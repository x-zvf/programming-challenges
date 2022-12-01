// Singly-linked lists are already defined with this interface:
// template<typename T>
// struct ListNode {
//   ListNode(const T &v) : value(v), next(nullptr) {}
//   T value;
//   ListNode *next;
// };
//
ListNode<int> * reverseNodesInKGroups(ListNode<int> * l, int k) {
     if (k < 2)
        return l;
    int i = 0;

    auto grouphead = &l;
    auto groupstart = l;
    auto aftergroupend = l;

    while (aftergroupend != nullptr) {
        aftergroupend = aftergroupend->next;
        if (++i < k)
            continue;

        auto end = groupstart;
        auto prev = aftergroupend;
        i++;
        while (--i) {
            auto np = groupstart->next;
            groupstart->next = prev;
            prev = groupstart;
            groupstart = np;
        }

        *grouphead = prev;
        grouphead = &end->next;
    }
    return l;
}

