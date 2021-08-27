#include <string>


std::string to_camel_case(std::string text) {
  std::string sol;
  bool n = false;
  for(auto it = text.begin(); it != text.end(); it++) {
    if(n) {
      sol += std::toupper(*it);
      n = false;
    } else if(*it == '-' || *it == '_') {
      n = true;
    } else {
      sol += *it;
    }
  }
  return sol;
}
