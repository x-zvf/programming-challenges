#include <bits/stdc++.h>
using namespace std;

int main() {
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    uint64_t n, i, j, count;
    cin >> n;

    count = n;
    j = n / 2;

    int64_t kangaroo[n];
    for(int i = 0; i<n; i++){
        cin >> kangaroo[i];
    }
    sort(kangaroo, kangaroo + n);

    for(int i = 0; i < n / 2; i++){
        for(; j < n; j++){
            if(kangaroo[j] >= kangaroo[i] * 2){
                j++;
                count--;
                break;
            }
        }
    }
    cout << count << "\n";
}