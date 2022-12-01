// Singly-linked lists are already defined with this interface:
// template<typename T>
// struct ListNode {
//   ListNode(const T &v) : value(v), next(nullptr) {}
//   T value;
//   ListNode *next;
// };
//

ListNode<int> *reverseList(ListNode<int> * l) {
    ListNode<int> *prev = nullptr;
    while(l != nullptr)
    {
        auto *tmp = l->next;
        l->next = prev;
        prev = l;
        l = tmp;
    }
    return prev;
}

ListNode<int> * addTwoHugeNumbers(ListNode<int> * a, ListNode<int> * b) {
    auto *result = new ListNode<int>(-1);
    auto rhead = result;
    auto ar = reverseList(a);
    auto br = reverseList(b);
    int carry = 0;
    while(ar != nullptr || br != nullptr) {
        if(ar != nullptr) {
            carry += ar->value;
            ar = ar->next;
        }
        if(br != nullptr) {
            carry += br->value;
            br = br->next;
        }

        rhead->next = new ListNode<int>(carry % 10000);
        carry = (int)carry/10000;
        rhead = rhead->next;
    }

    if(carry > 0)
        rhead->next = new ListNode<int>(carry);

    return reverseList(result->next);
}

