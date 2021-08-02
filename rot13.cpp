#include <string>
using namespace std;

string rot13(string msg)
{
  stringstream ss;
  for(char c : msg) {
    if(c >= 'a' && c <= 'z')
      ss << (char)('a' + (((c - 'a')+13)%26));
    else if(c >= 'A' && c <= 'Z')
      ss << (char)('A' + (((c - 'A')+13)%26));
    else
      ss << c;
  }
  return ss.str();
}