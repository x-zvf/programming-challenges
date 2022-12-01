#include <bits/stdc++.h>
using namespace std;
int64_t mod = 1e9 + 7;
int main()
{
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    int n, m;
    cin >> n >> m;
    int p[2 * n + 1], c[2 * n + 1] = {}, u, v;

    for (int i = 1; i <= 2 * n; i++)
        cin >> p[i];
    for (int i = 1; i <= m; i++)
    {
        cin >> u >> v;
        c[u] = v;
        c[v] = u;
    }
    int t, l = 0;
    cin >> t;
    for (int i = 1; i <= 2 * n; i++)
    {
        if ((i + t) % 2)
        {
            cin >> l;
            p[l] = 0;
        }
        else
        {
            if (c[l] && p[c[l]])
            {
                p[c[l]] = 0;
                cout << c[l] << endl;
            }
            else
            {
                int id = 0, maxx = 0;
                for (int i = 1; i <= 2 * n; i++)
                {
                    if (p[i] > maxx && c[i])
                    {
                        maxx = p[i];
                        id = i;
                    }
                }
                if (id == 0)
                {
                    maxx = 0;
                    for (int i = 1; i <= 2 * n; i++)
                    {
                        if (p[i] > maxx)
                        {
                            maxx = p[i];
                            id = i;
                        }
                    }
                }
                p[id] = 0;
                cout << id << endl;
            }
        }
    }
    cout.flush();
    return 0;
}