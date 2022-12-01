#include <bits/stdc++.h>

using namespace std;

int64_t grams[100000 + 10], cost[100000 + 10];

int main() {
    int64_t n = 0, k = 0, a = 0, sol = 0;
    //scanf("%I64ld %I64ld", &n, &k);
    cin >> n >> k;
    for(int i=0; i<n; i++){
        //scanf("%I64ld", &grams[i]);
        cin >> grams[i];
    }
    cin >> a;
    //scanf("%I64ld", &a);
    for(int i=0; i<n; i++){
        //scanf("%I64ld", &cost[i]);
        cin >> cost[i];
    }
    priority_queue<uint64_t, vector<uint64_t>, greater<int>> pq;

    for(int i = 0; i < n; i++) {
        pq.push(cost[i]);
        while(k < grams[i] && pq.size() > 0){
            k += a;
            sol += pq.top();
            pq.pop();
        }
        if(k < grams[i]) {
            printf("-1\n");
            return 0;
        }
    }
    //printf("%I64ld\n", sol);
    cout << sol << "\n";
    return 0;
}