#include <string>

std::string highestScoringWord(const std::string &str)
{
  std::string bestword = "";
  std::string cw = "";
  int highscore = 0;
  int score = 0;
  
  for(char c : str) {
    if(c == ' ') {
      if(score > highscore) {
        highscore = score;
        bestword = cw;
      }
      cw = "";
      score = 0;
    } else {
      score += c - 'a' + 1;
      cw += c;
    }
  }
  if(score > highscore) {
    highscore = score;
    bestword = cw;
  }
  
  return bestword;
}
