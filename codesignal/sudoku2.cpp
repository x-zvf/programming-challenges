bool sudoku2(vector<vector<char>> grid) {
    std::unordered_set<char> seen;
    for(const auto &row : grid) {
        for(const char &cell  : row) {
            if(cell != '.' && seen.count(cell))
                return false;
            seen.insert(cell);
        }
        seen.clear();
    }
    
    for(int col = 0; col < 9; col++) {
        for(int row = 0; row < 9; row++){
            const auto& cell = grid[row][col];
            if(cell != '.' && seen.count(cell))
                return false;
            seen.insert(cell);
        }
        seen.clear();
    }
    for(int sgr = 0; sgr < 3; sgr++) {
        for(int sgc = 0; sgc < 3; sgc++) {
            
            for(int col = 0; col < 3; col++) {
                for(int row = 0; row < 3; row++){
                    
                    const auto& cell = grid[3*sgr + row][3*sgc + col];
                    if(cell != '.' && seen.count(cell))
                        return false;
                    seen.insert(cell);
                }
            }
            
            seen.clear();
        }
    }
    return true;
}
