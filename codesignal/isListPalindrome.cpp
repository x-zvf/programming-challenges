// Singly-linked lists are already defined with this interface:
// template<typename T>
// struct ListNode {
//   ListNode(const T &v) : value(v), next(nullptr) {}
//   T value;
//   ListNode *next;
// };
//
bool isListPalindrome(ListNode<int> * l) {
    //find midpoint
    auto endptr = l, snd_half_ptr = l, snd_half_prev = l, end_f_h = l;
    while(endptr != nullptr && endptr->next != nullptr) {
        endptr = endptr->next->next;
        snd_half_prev = snd_half_ptr;
        snd_half_ptr = snd_half_ptr->next;
    }
    //odd number elems
    if(endptr != nullptr)
    {
        end_f_h = snd_half_prev;
        snd_half_ptr = snd_half_ptr->next;
    } else {
        end_f_h = snd_half_ptr;
    }

    //reverse snd half

    ListNode<int> *next = nullptr, *prev = nullptr;

    while(snd_half_ptr != nullptr)
    {
        next = snd_half_ptr->next;
        snd_half_ptr->next = prev;
        prev = snd_half_ptr;
        snd_half_ptr = next;
    }
    snd_half_ptr = prev;

    //compare
    while(l != end_f_h) {
        if(l->value != snd_half_ptr->value)
            return false;
        l = l->next;
        snd_half_ptr = snd_half_ptr->next;
    }
    return true;
}

