bool areFollowingPatterns(vector<string> strings, vector<string> patterns) {
    map<string, string> string_pattern;
    map<string, string> pattern_string;

    for(int i = 0; i<patterns.size(); i++) {
        if((string_pattern.count(strings[i])  && string_pattern[strings[i]]  != patterns[i])
        || (pattern_string.count(patterns[i]) && pattern_string[patterns[i]] != strings[i]))
            return false;

        string_pattern[strings[i]] = patterns[i];
        pattern_string[patterns[i]] = strings[i];
    }
    return true;
}

