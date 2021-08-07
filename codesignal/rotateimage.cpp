vector<vector<int>> rotateImage(vector<vector<int>> a) {
    for(int i = 0; i<a.size(); i++) {
        for(int j=i; j < a.size(); j++){
            auto tmp = a[i][j];
            a[i][j] = a[j][i];
            a[j][i] = tmp;
        }
    }
    for(auto& row : a) {
        std::reverse(row.begin(), row.end());
    }
    return a;
}
