bool isValid(vector<int> board, int n, int col, int y) {
    for(int a : board) {
        if(a == y + 1)
            return false;
    }
    for(int x = col-1; x >= 0; x--) {
        if(board[x] == (y-(col-x)) + 1 || board[x] == (y+(col-x)) + 1)
            return false;
    }
    return true;
}

void backtrack(vector<vector<int>> &sol, vector<int> current, int n, int col) {
    if(col == n) {
        sol.push_back(current);
        return;
    }
    for(int y = 0; y < n; y++) {
        if(isValid(current, n, col, y)) {
            current.push_back(y+1);
            backtrack(sol, current, n, col+1);
            current.pop_back();
        }
    }
}

vector<vector<int>> nQueens(int n) {
    vector<vector<int>> sol;
    backtrack(sol, {}, n, 0);
    return sol;
}

