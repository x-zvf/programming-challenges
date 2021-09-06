int houseRobber(vector<int> nums) {
    if(nums.empty())
        return 0;
    int dp[nums.size() + 1];
    for(int i = 2; i<nums.size()+1; i++)
        dp[i] = 0;
    dp[0] = 0;
    dp[1] = nums[0];
    for(int i = 2; i<nums.size()+1; i++) {
        dp[i] =std::max(dp[i-1], nums[i-1] + dp[i-2]);
    }
    return dp[nums.size()];
}
