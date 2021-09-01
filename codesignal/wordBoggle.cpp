bool backtrack_word(vector<vector<char>> &board, string &word, unordered_set<int> &used_pos, int row, int col, int cindex) {
    if(cindex == word.size())
        return true;
    if(board[row][col] != word[cindex] || used_pos.count((row*board[0].size())+col))
        return false;
    used_pos.insert((row*board[0].size())+col);
    vector<pair<int,int>> moves = {
        {-1,-1},
        {-1, 0},
        {-1, 1},
        {0, -1},
        {0,  1},
        {1, -1},
        {1,  0},
        {1,  1},
    };
    for(auto [dcol, drow] : moves) {
        int nrow = row + drow;
        int ncol = col + dcol;

        if(nrow < 0 || ncol < 0 || nrow >= board.size() || ncol >= board[0].size())
            continue;
        if(backtrack_word(board, word, used_pos, nrow, ncol, cindex+1)) {
            used_pos.erase((row*board[0].size())+col);
            return true;
        }
    }
    used_pos.erase((row*board[0].size())+col);
    return false;
}
vector<string> wordBoggle(vector<vector<char>> board, vector<string> words) {
    set<string> sol;
    for(auto &word : words) {
        for(int i=0; i<board.size()*board[0].size(); i++) {
            unordered_set<int> used_pos;
            if(backtrack_word(board, word, used_pos, i/board[0].size(), i%board[0].size(), 0)) {
                sol.insert(word);
                break;
            }
        }
    }
    vector<string> ret;
    for(auto &s : sol)
        ret.push_back(s);
    return ret;
}

