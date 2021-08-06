#include <regex>

int countSmileys(std::vector<std::string> arr)
{
  std::regex reg("[:;][-~]{0,1}[)D]");
  return std::count_if(arr.begin(), arr.end(), [reg](std::string sm){return std::regex_match(sm,reg);});
}