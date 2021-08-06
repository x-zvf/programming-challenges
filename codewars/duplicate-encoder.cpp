#include <string>
std::string duplicate_encoder(const std::string& w){
  std::string word(w);
  std::unordered_map<char,int> counts;
  for(char c : word)
    counts[std::tolower(c)]++;
  for(char& c : word)
    c = (counts[std::tolower(c)] > 1) ? ')' : '(';
  return word;
}