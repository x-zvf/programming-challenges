#include <string>

class RevRot
{
public:
    static std::string revRot(const std::string &strng, unsigned int sz)
    {
      if(sz <= 0 || sz > strng.length())
        return "";
      std::vector<std::string> chunks;
      std::string cs;
      for(char c : strng) {
        cs += c;
        if(cs.length() == sz) {
          chunks.push_back(cs);
          cs = "";
        }
      }
      std::stringstream sol;
      for(std::string& chunk : chunks) {
        int cubesum = 0;
        for(char digit : chunk) {
          int dval = digit - '0';
          cubesum += dval * dval * dval;
        }
        if(cubesum % 2 == 0) {
          std::reverse(chunk.begin(), chunk.end());
          sol << chunk;
        } else {
          for(int i = 1; i < sz; i++)
            sol << chunk[i];
          sol << chunk[0];
        }
      }
      return sol.str();
    }
};
