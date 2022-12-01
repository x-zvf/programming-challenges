bool isCryptSolution(vector<string> crypt, vector<vector<char>> solution) {
    std::unordered_map<char,char> solmap;
    for(const auto &mapping : solution)
        solmap[mapping[0]] = mapping[1];
    
    for(auto &word : crypt){
        for(char &c : word)
            c = solmap[c];
        if(word.length() > 1 && word[0] == '0')
            return false;        
    }

    int numbers[3];
    for(int i = 0; i<3; i++) {
        numbers[i] = 0;
        int dec = 1;
        for(auto it = crypt[i].rbegin(); it != crypt[i].rend(); it++){
            numbers[i] += dec * (*it - '0');
            dec *= 10;
        }
        std::cout << numbers[i] << std::endl;
    }
    
    return numbers[0] + numbers[1] == numbers[2];    
}
