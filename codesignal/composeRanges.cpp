#include <limits>
vector<string> composeRanges(vector<int> nums) {
    if(nums.empty())
        return {};
    int last = nums.front();
    int cmin = nums.front();
    nums.push_back(std::numeric_limits<int>::min());
    vector<string> sol;
    for(int x : nums) {
        if(x > last + 1 || x == std::numeric_limits<int>::min()) {
            stringstream ss;
            ss << cmin;
            if(last > cmin)
                ss << "->" <<last;
            sol.push_back(ss.str());
            cmin = x;
        }
        last = x;
    }
    return sol;
}

