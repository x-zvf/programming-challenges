int mapDecoding(string message) {
    int mod = 1000000007;
    if(message.length() == 0)
        return 1;
    vector<int> dp(message.length() + 1);
    dp[0] = 1;
    dp[1]= message[0] == '0' ? 0 : 1;

    for(int i = 2; i <= message.length(); i++){
        int a = std::stoi(message.substr(i-1, 1));
        if(a != 0)
            dp[i] = (dp[i] +dp[i-1]) % mod;

        int b = std::stoi(message.substr(i-2, 2));
        if(b > 9 && b <= 26)
            dp[i] = (dp[i] + dp[i-2]) % mod;
    }
    return dp.back();
}
