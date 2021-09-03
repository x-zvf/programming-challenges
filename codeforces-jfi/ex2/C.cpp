#include <bits/stdc++.h>
using namespace std;

int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    int64_t n, m, k;
    cin >> n >> m >> k;

    int64_t lower_bound = 1;
    int64_t upper_bound = n * m;
    int64_t ans = 0;

    while (lower_bound <= upper_bound)
    {
        int64_t mid = (lower_bound + upper_bound) / 2;
        int64_t n_lower = 0;
        int64_t n_equal = 0;

        for (int64_t i = 1; i <= n; i++)
        {
            //int64_t minn = i;
            int64_t maxn = m * i;
            if (mid <= maxn)
            {
                n_lower += (mid / i);
                if ((mid % i) == 0)
                {
                    n_equal++;
                    n_lower -= 1;
                }
            }
            else
                n_lower += m;
        }

        if (n_lower >= k)
            upper_bound = mid - 1;
        else if ((n_lower + n_equal) >= k)
        {
            ans = mid;
            break;
        }
        else
            lower_bound = mid + 1;
    }

    cout << ans << endl;
}