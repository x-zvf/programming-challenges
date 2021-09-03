#include <bits/stdc++.h>
using namespace std;

int main() {
    cin.tie(0);
    cout.tie(0);
    ios_base::sync_with_stdio(false);

    int n, t;
    cin >> n >> t;
    vector<int> v(n);
    for(int i = 0; i<n;i++){
        cin >> v[i];
    }
    
    int l = 0, r = 0;
    int ans = 0;
    int sum = v[l];
    
    while(l<n && r<n){
    	if(sum<t){
    		if(r<(n-1) && sum+v[r+1]<=t){
    			sum+=v[++r];
    		} else if(l<(n-1)) {
	    		sum-=v[l++];
    		}
    	} else {
    		sum-=v[l++];
    	}
    	ans = max(ans, r-l+1);
    	if(r>=(n-1) || l>=(n-1)) break;
    }
    cout << ans << "\n";
    return 0;
}