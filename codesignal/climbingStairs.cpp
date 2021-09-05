unordered_map<int, int> memo;

int climbingStairs(int n) {
    if(n <= 1)
        return 1;
    if(memo.count(n)) {
        return memo[n];
    }
    int steps = climbingStairs(n-2) + climbingStairs(n-1);
    memo[n] = steps;
    return steps;
}

