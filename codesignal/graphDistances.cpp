#include <limits>
vector<int> graphDistances(vector<vector<int>> g, int s) {
    vector<int> sol(g.size(), numeric_limits<int>::max());
    sol[s] = 0;
    priority_queue<pair<int,int>, vector<pair<int,int>>> q;
    q.push({0,s});
    while(!q.empty()) {
        auto np = q.top();
        q.pop();
        for(int i=0; i < g.size(); i++) {
            if(g[np.second][i] >= 0 && sol[i] > sol[np.second] + g[np.second][i]) {
                sol[i] = sol[np.second] + g[np.second][i];
                q.push({sol[i],i});
            }
        }
    }
    return sol;
}
