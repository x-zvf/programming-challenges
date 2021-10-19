/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int sum, carry = 0;
        auto r = new ListNode(-1);
        auto head = r;
        while(carry || l1 != nullptr || l2 != nullptr) {
            sum = carry;
            if(l1 != nullptr) {
                sum += l1->val;
                l1 = l1->next;
            }
            if(l2 != nullptr) {
                sum+= l2->val;
                l2 = l2->next;
            }
            r->next = new ListNode(sum % 10);
            r = r->next;
            carry = sum / 10;
        }
        return head->next;
    }
};
