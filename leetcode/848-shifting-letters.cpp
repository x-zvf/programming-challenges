class Solution {
public:
    string shiftingLetters(string s, vector<int>& shifts) {
        int prev = 0, cshift = 0;
        for(int i = s.length()-1; i>=0; i--) {
            cshift = (prev + shifts[i]) % 26;
            s[i] = 'a' + (((s[i]-'a')+cshift) % 26);
            prev = cshift;
        }
        return s;
    }
};
