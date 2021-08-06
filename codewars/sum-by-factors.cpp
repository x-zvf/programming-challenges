#include <bits/stdc++.h>

class SumOfDivided
{
public:
    static std::string sumOfDivided(std::vector<int> &lst){
      std::set<int> pf;
      for(int i : lst) {
        i = std::abs(i);
        int isqr = std::sqrt(i);
        while(i % 2 == 0) {
          pf.insert(2);
          i /= 2;
        }
        for(int p = 3; p <= isqr; p+=2) {
          if(i % p == 0) 
            pf.insert(p);
          while(i % p == 0)
            i /= p;
        }
        if(i > 2) 
          pf.insert(i);
      }
      std::stringstream ss;
      for(const auto p : pf) {
        int sum = 0;
        for(int i : lst) {
          if(i % p == 0)
            sum += i;
        }
        ss << '(' << p << ' ' << sum << ')';
      }
      return ss.str();
    }
};
