bool containsDuplicates(vector<int> a) {
    unordered_set<int> s;
    for(int x : a)
        s.insert(x);
    return a.size() != s.size();
}
