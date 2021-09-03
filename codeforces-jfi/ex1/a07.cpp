#include <cstdio>
#include <algorithm>

using namespace std;

int a[100000 + 10], incr[100000 + 10], dec[100000 + 10];

int main()
{
    int n, m;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= n; i++)
    {
        scanf("%d", &a[i]);
    }
    incr[n] = n;
    dec[n] = n;
    for (int i = n - 1; i >= 1; i--)
    {
        if (a[i] > a[i + 1])
        {
            incr[i] = i;
            dec[i] = dec[i + 1];
        }
        else if (a[i] < a[i + 1])
        {
            incr[i] = incr[i + 1];
            dec[i] = i;
        }
        else
        {
            incr[i] = incr[i + 1];
            dec[i] = dec[i + 1];
        }
    }

    for (int i = 0; i < m; i++)
    {
        int l, r;
        scanf("%d%d", &l, &r);
        if (dec[incr[l]] >= r)
            printf("Yes\n");
        else
            printf("No\n");
    }
    return 0;
}