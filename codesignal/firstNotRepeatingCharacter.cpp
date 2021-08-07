char firstNotRepeatingCharacter(string s) {
    int counts[26];
    for(int i = 0; i<26; i++)
        counts[i] = 0;
    
    for(char c : s)
        counts[c-'a']++;
    
    for(char c : s)
        if(counts[c-'a'] < 2)
            return c;
    
    return '_';
}
