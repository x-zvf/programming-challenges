class Solution {
public:    
    bool isHappy(int n) {            
        while(true){
            if(n <= 9)
                // 7 leads to a one aswell.
                return n == 1 || n == 7;

            int sum = 0;
            while(n > 0){
                int d = (n%10);
                sum += d * d;
                n/=10;
            }
            n = sum;
        }
        return false;
    }
};

/* naive solution:
class Solution {
public:
    bool isHappy(int n) {
        std::set<int> nums;
        while(n != 1) {
            if(nums.count(n) > 0)
                return false;
            int sum = 0;
            for(char c : std::to_string(n)) {
                int d = c - '0';
                sum += d * d;
            }
            nums.insert(n);
            n = sum;
        }
        return true;
    }
};
*/