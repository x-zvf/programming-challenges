void foo(vector<vector<int>> &out, vector<int> &csteps, int k, int rem) {
    if(rem == 0) {
        vector<int> cp = csteps;
        out.push_back(cp);
        return;
    }
    for(int s = 1; s <= k; s++) {
        if(s > rem)
            break;
        csteps.push_back(s);
        foo(out, csteps, k, rem-s);
        csteps.pop_back();
    }
}
vector<vector<int>> climbingStaircase(int n, int k) {
    vector<vector<int>> solution;
    vector<int> csteps;
    if(n == 0 || k == 0) {
        solution.push_back({});
        return solution;
    }
    foo(solution, csteps, k, n);
    return solution;
}
