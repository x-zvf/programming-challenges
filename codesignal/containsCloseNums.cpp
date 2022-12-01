bool containsCloseNums(vector<int> nums, int k) {
    unordered_map<int,int> numpos;
    for(int i=0; i<nums.size(); i++)
    {
        if(numpos.count(nums[i]) && i-numpos[nums[i]] <= k)
            return true;
        numpos[nums[i]] = i;
    }
    return false;
}

