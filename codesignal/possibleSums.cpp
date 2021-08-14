int possibleSums(vector<int> coins, vector<int> quantity) {
    unordered_set<int> allsums;
    allsums.insert(0);

    for(int i = 0; i < coins.size(); i++) {
        auto sumsuntilnow = allsums;
        for(auto sum : sumsuntilnow) {
            for(int q=1; q<=quantity[i]; q++)
                allsums.insert(sum + q*coins[i]);
        }
    }
    return allsums.size() - 1;
}

