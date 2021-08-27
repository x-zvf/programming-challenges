bool is_isogram(std::string str) {
  int counts[26];
  for(int i=0; i<26; i++) counts[i] = 0;

  for(char c : str) {
    if(counts[std::tolower(c)-'a']++)
      return false;
  }

  return true;
}
