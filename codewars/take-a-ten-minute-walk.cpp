#include<vector>

bool isValidWalk(std::vector<char> walk) {
  int x = 0, y = 0;
  if(walk.size() != 10) return false;
  
  for(char c : walk) {
    switch(c) {
        case 'n':
          y--;
        break;
        case 's':
          y++;
        break;
        case 'w':
          x--;
        break;
        case 'e':
          x++;
        break;
    }
  }
  return (!x && !y);
}