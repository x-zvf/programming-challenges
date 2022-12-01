// Singly-linked lists are already defined with this interface:
// template<typename T>
// struct ListNode {
//   ListNode(const T &v) : value(v), next(nullptr) {}
//   T value;
//   ListNode *next;
// };
//
ListNode<int> * mergeTwoLinkedLists(ListNode<int> * l1, ListNode<int> * l2) {
    ListNode<int> *res = new ListNode<int>(-1);
    auto ret = res;
    while(l1 != nullptr && l2 != nullptr) {
        if(l1->value <= l2->value) {
            res->next = l1;
            l1 = l1->next;
        } else {
            res->next = l2;
            l2 = l2->next;
        }
        res = res->next;
    }
    if(l1 != nullptr)
        res->next = l1;
    if(l2 != nullptr)
        res->next = l2;

    return ret->next;
}

