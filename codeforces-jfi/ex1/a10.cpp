#include <bits/stdc++.h>
using namespace std;

int main() {
    cin.tie(0);
    ios_base::sync_with_stdio(false);

	int t;
	cin >> t;
	while(t--) {
		int n, k;
		cin >> n >> k;
		vector<int> a(n);
		for(int i = 0; i < n; i++) 
			cin >> a[i];
		if(k == 1) {
			if(a[0] != a.back()) {
				cout << -1 << endl;
			} else {
				cout << 1 << endl;
			}
			continue;
		}
		n = unique(a.begin(), a.end()) - a.begin();
		if(n <= k) {
			cout << 1 << endl;
		} else {
			n--;
			cout << (n + k - 2) / (k - 1) << endl;
		}
	}
    return 0;
}