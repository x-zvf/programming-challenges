#include <bits/stdc++.h>
using namespace std;

int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);
    uint64_t n;
    cin >> n;
    vector<uint64_t> as(n);
    uint64_t s = 0, ges = 0;
    for(int i = 0; i<n; i++) {
        uint64_t x;
        cin >> x;
        as[i] = x;
        s = max(s,x);
        ges += x;
    }
    bool ug = ges %(n-1) != 0;
    uint64_t c = max(s, ges / (n-1) + ug);
    cout << c << "\n";
    return 0;
}