// Singly-linked lists are already defined with this interface:
// template<typename T>
// struct ListNode {
//   ListNode(const T &v) : value(v), next(nullptr) {}
//   T value;
//   ListNode *next;
// };
//
ListNode<int> * rearrangeLastN(ListNode<int> * l, int n) {
    if (n<1)
        return l;

    auto endptr = l;
    auto slowptr = l;
    while(n-- > 0 && endptr != nullptr)
        endptr = endptr->next;
    if(endptr == nullptr)
        return l;
    std::cout << "endptr->value=" << endptr->value <<"\n";
    while(endptr->next!=nullptr){
        endptr = endptr->next;
        slowptr = slowptr->next;
    }
    //slowptr = slowptr->next;
    std::cout << " endptr->value=" << endptr->value
              << "\n slowptr->value=" << slowptr->value << "\n";
    //slowptr should be at the start of the rearrange part now

    endptr->next = l;
    auto ret = slowptr->next;
    slowptr->next = nullptr;
    return ret;
}

