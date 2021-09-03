#include <bits/stdc++.h>
using namespace std;

map<int64_t, vector<int64_t>> adjl;
map<int64_t, bool> visited;

void dfs(int64_t x)
{
    if (visited[x])
        return;

    cout << x << " ";
    visited[x] = true;

    for (int64_t y : adjl[x])
        dfs(y);
}

int main()
{
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    int64_t n;
    cin >> n;

    for (int i = 0; i < n; i++)
    {
        int64_t a, b;
        cin >> a >> b;
        adjl[a].push_back(b);
        adjl[b].push_back(a);

        visited[a] = false;
        visited[b] = false;
    }

    int64_t x;
    auto it = adjl.begin();
    // find the point with only one adj
    while (it != adjl.end())
    {
        if (it->second.size() == 1)
        {
            x = it->first;
        }
        it++;
    }

    dfs(x);

    return 0;
}