#include <string>
int find_short(std::string str)
{
  int minlen = std::numeric_limits<int>::max();
  int clen = 0;
  for(char c : str) {
    if(c == ' '){
      if(clen < minlen) minlen = clen;
      clen = 0;
    } else {
      clen++;
    }
  }
  if(clen < minlen) minlen = clen;
  return minlen;
}