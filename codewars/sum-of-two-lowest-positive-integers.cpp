#include <vector>

long sumTwoSmallestNumbers(std::vector<int> numbers)
{
    long m1 = std::numeric_limits<long>::max();
    long m2 = m1;
  
    for(int n : numbers) {
      if(n < m1) {
        if(m1 < m2) m2 = m1;
        m1 = n;
      }
      else if(n < m2) {
        if(m2 < m1) m1 = m2;
        m2 = n;
      }
    }
    return m1 + m2;
}