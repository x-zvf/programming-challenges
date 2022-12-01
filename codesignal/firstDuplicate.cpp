int firstDuplicate(vector<int> a) {
    for(int i = 0; i<a.size(); i++) {
        if(a[std::abs(a[i])-1] < 0)
            return std::abs(a[i]);
        else
            a[std::abs(a[i])-1] *= -1;
    }
    return -1;
}
