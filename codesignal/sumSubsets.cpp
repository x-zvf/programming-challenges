void backtrack(vector<vector<int>> &sol, vector<int> &curr, const vector<int> &arr, int num, int s, int idx) {
    for(int i = idx; i < arr.size(); i++) {
        int n = s + arr[i];
        if(n > num)
            return;
        curr.push_back(arr[i]);
        if(n == num) {
            vector<int> a = curr;
            bool found = false;
            // don't do this kids.
            // This changes the solution from O(ok) to O(fuck)
            for(auto &s : sol) {
                if(s == a) {
                    found = true;
                    break;
                }
            }
            if(!found)
                sol.push_back(a);
            curr.pop_back();
            return;
        }
        backtrack(sol, curr, arr, num, n, i+1);
        curr.pop_back();
    }
}
vector<vector<int>> sumSubsets(vector<int> arr, int num) {
    vector<vector<int>> sol;
    vector<int> curr;
    backtrack(sol, curr, arr, num, 0, 0);
    if(sol.empty())
        sol.push_back({});
    return sol;
}

