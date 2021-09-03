#include <bits/stdc++.h>
using namespace std;

int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    uint64_t n, k;
    cin >> n >> k;

    int64_t as[100003];
    for (int i = 0; i < n; i++)
        cin >> as[i];

    int64_t sum = 0;

    pair<int, int> ans;
    ans.first = 0;
    ans.second = 0;

    sort(as, as + n);


    for (int i = 0, j = 0; i < n; i++)
    {
        sum += as[i];
        while ((i - j + 1LL) * as[i] - sum > k)
        {
            sum -= as[j++];
        }
        if (ans.second < i - j + 1LL)
        {
            ans = make_pair(as[i], i - j + 1LL);
        }
    }

    cout << ans.second << ' ' << ans.first << endl;

    return 0;
}