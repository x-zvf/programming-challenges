int root(int x, vector<int>& l) {
    if (l[x] < 0)
        return x;
    l[x] = root(l[x], l);
    return l[x];
}

std::string swapLexOrder(std::string str, std::vector<std::vector<int> > pairs) {
    vector<int> l(str.length());
    for (int i = 0; i < str.length(); i++)
        l[i] = -1;

    for (int i = 0; i < pairs.size(); i++) {
        int a = root(pairs[i][0] - 1, l);
        int b = root(pairs[i][1] - 1, l);
        if (a != b) {
            if (l[a] > l[b])
                swap(a, b);
            l[a] += l[b];
            l[b] = a;
        }
    }

    multiset<char> stringset[str.length()];

    for (int i = 0; i < str.length(); i++) {
        int x = root(i, l);

        stringset[x].insert(str[i]);
    }

    for (int i = 0; i < str.length(); i++) {
        int x = root(i, l);

        str[i] = *stringset[x].rbegin();
        stringset[x].erase(--stringset[x].end());
    }

    return str;
}
