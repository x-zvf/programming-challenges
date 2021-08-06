#include<string>


bool scramble(const std::string& s1, const std::string& s2){
    std::unordered_map<char,int> counts;
    for(char c : s1)
      counts[c]++;
  
    for(char c : s2)
      if(--counts[c] < 0) 
        return false;

    return true;
}