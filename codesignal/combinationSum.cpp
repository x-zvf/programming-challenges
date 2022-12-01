void backtrack(vector<vector<int>> &sol, vector<int> &a, vector<int> csol, int startindex, int sum) {
    if(sum == 0) {
        sol.push_back(csol);
        return;
    }
    if(startindex == a.size())
        return;
    while(sum >= 0) {
        backtrack(sol, a, csol, startindex + 1, sum);
        csol.push_back(a[startindex]);
        sum -= a[startindex];
    }
}
string combinationSum(vector<int> a, int sum) {
    std::set<int> numbers;
    for(int x : a)
        numbers.insert(x);
    a.clear();
    for(int x : numbers)
        a.push_back(x);
    std::cout << "a=[";
    for(int x : a)
        std::cout << x<< ", ";
    std::cout << "]\n";

    vector<vector<int>> sol;
    backtrack(sol, a, {}, 0, sum);
    if(sol.empty())
        return "Empty";

    stringstream ret;
    for(auto it = sol.rbegin(); it != sol.rend(); it++) {
        ret << "(";
        for(int i = 0; i < (*it).size()-1; i++)
            ret << (*it)[i] << ' ';
        ret << (*it).back() << ')';
    }
    return ret.str();
}

