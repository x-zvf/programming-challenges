#include <bits/stdc++.h>
using namespace std;

int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    int64_t n, k, count;
    cin >> count >> n;

    double x = n;

    for (int i = 0; i < count * 2; i++)
    {
        int t;
        cin >> t;
        if (t == 1)
        {
            cout << -1;
            return 0;
        }
        x *= (double)t / (t - 1);
    }
    //cout << x - n;
    printf("%lf",x-n);
    return 0;
}