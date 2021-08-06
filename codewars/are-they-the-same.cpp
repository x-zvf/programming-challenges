#include <vector>

class Same {
public :
    static bool comp(std::vector<int>&a, std::vector<int>&b) {
      std::transform(a.begin(), a.end(), a.begin(), [](auto n){return n*n;});
      std::sort(a.begin(), a.end());
      std::sort(b.begin(), b.end());
      return a == b;
    }
};
