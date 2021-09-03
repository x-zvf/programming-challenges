#include <bits/stdc++.h>
using namespace std;

int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    int64_t n, d;
    cin >> n >> d;

    vector<int64_t> v(n);
    for (int64_t i = 0; i < n; i++)
        cin >> v[i];

    int64_t ans = 0;
    for (uint64_t i = 0; i < n; i++)
    {
        int64_t upb_of = v[i] + d;
        uint64_t idx =
            upper_bound(v.begin(), v.end(), upb_of) - v.begin();
        //dst
        int64_t k = idx - i - 1;
        ans += (k * (k - 1)) / 2;
    }
    cout << ans << endl;

    return 0;
}