unordered_map<int,int> memo = {{2,5}, {3,11}};
int fillingBlocks(int n) {
    if(n < 2)
        return 1;
    if(memo.count(n))
        return memo[n];
    //fiddly math worked out on paper
    int res = fillingBlocks(n - 1) + 5 * fillingBlocks(n - 2) + fillingBlocks(n - 3) - fillingBlocks(n - 4);
    memo[n] = res;
    return res;
}
