class Solution {
public:
    vector<int> sortArrayByParityII(vector<int>& nums) {
        int odd = 1;
        for(int  i = 0; i<nums.size(); i+=2)
        {
            if(nums[i] % 2)
            {
                while(nums[odd] % 2) odd += 2;
                int tmp = nums[i];
                nums[i] = nums[odd];
                nums[odd] = tmp;
            }
        }
        return nums;
    }
};
