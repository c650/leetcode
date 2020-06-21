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
        std::function<ListNode*(ListNode*,ListNode*,int)> go = [&](auto l1, auto l2, auto carry) {
            if (!l1 && !l2) {
                return carry ? 
                    new ListNode(carry)
                    : nullptr;
            }
            
            const auto val = (l1 ? l1->val : 0) + (l2 ? l2->val : 0) + carry;
            
            return new ListNode(val % 10, go(l1 ? l1->next : nullptr, l2 ? l2->next : nullptr, val / 10));
        };
        
        return go(l1, l2, 0);
    }
};
