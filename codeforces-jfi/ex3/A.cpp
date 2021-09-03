#include <iostream>
using namespace std;
int x, y, g[52], s[52], t[52], num[52];
int f(int x)
{
    if (g[x] != x)
        g[x] = f(g[x]);
    return g[x];
}
int main()
{
    int i, j, n, m;
    cin >> n >> m;
    for (i = 1; i <= n; i++)
        g[i] = i;
    for (i = 1; i <= m; i++)
    {
        cin >> x >> y;
        g[f(x)] = f(y);
    }
    for (i = 1; i <= n; i++)
        num[f(i)]++;
    for (i = 1; i <= n; i++)
        if (num[i] > 3)
        {
            cout << -1 << endl;
            return 0;
        }
    for (i = 1; i <= n; i++)
    {
        if (num[i] == 1)
            s[++s[0]] = i;
        if (num[i] == 2)
            t[++t[0]] = i;
    }
    if (t[0] > s[0])
    {
        cout << -1;
    }
    else
    {
        for (i = 1; i <= t[0]; i++)
            g[s[i]] = t[i];
        for (i = t[0] + 1; i <= s[0]; i += 3)
        {
            g[s[i + 2]] = s[i];
            g[s[i + 1]] = s[i];
        }
        for (i = 1; i <= n; i++)
        {
            if (f(i) == i)
            {
                for (j = 1; j <= n; j++)
                    if (f(j) == i)
                        cout << j << " ";
            }
        }
    }
    return 0;
}
