bool valid_braces(std::string braces) 
{
  std::stack<char> stack;
  std::map<char,char> matching = {{'(', ')'},
                                  {'[', ']'},
                                  {'{', '}'}};
  for(char c : braces) {
    if(c == '(' || c == '[' || c == '{') {
      stack.push(c);
    } else {
      if(stack.empty()) return false;
      char b = stack.top();
      stack.pop();
      char correct = matching[b];
      if(c != correct) return false;
    }
  }
  return stack.empty();
}