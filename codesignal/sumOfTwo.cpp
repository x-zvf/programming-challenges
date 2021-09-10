bool sumOfTwo(vector<int> a, vector<int> b, int v) {
    unordered_set<int> bset(b.begin(), b.end());
    for(int x : a){
        if(bset.count(v-x))
            return true;
    }
    return false;
}

