// Singly-linked lists are already defined with this interface:
// template<typename T>
// struct ListNode {
//   ListNode(const T &v) : value(v), next(nullptr) {}
//   T value;
//   ListNode *next;
// };
//
ListNode<int> * removeKFromList(ListNode<int> * l, int k) {
    if(l == nullptr)
        return l;

    while(l != nullptr && l->value == k)
        l = l->next;
    auto *list = l;
    while(l != nullptr && l->next != nullptr) {
        if(l->next->value == k)
            l->next = l->next->next;
        else
            l = l->next;
    }
    return list;
}

