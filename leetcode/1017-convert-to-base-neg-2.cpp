class Solution {
public:
    string baseNeg2(int n) {
        string sol;
        int odd = 1;
        while (n > 0) {
            int lbit = n & 0x1;
            n = (n - (lbit *odd)) / 2;
            odd *= -1;
            sol = to_string(lbit) + sol;
        }
        return sol.length() == 0 ? "0" : sol;
    }
};
