#include<cmath>
class DigPow
{
public:
  static int digPow(int n, int p) {
    long long sum = 0;
    int num = n;
    std::vector<int> digits;
    while(n>0) {
      int digit = n % 10;
      n /= 10;
      digits.push_back(digit);
    }
    for(auto rit = digits.rbegin(); rit != digits.rend(); ++rit)
      sum += std::pow(*rit, p++);
    return sum % num == 0 ? sum/num : -1;
  }
};