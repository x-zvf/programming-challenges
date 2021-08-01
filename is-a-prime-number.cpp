#include<cmath>
bool isPrime(int num) {
  if(num < 2) return false;
  int factor = 2;
  int root = std::floor(std::sqrt(num));
  while(factor <= root) {
    if(!(num % factor)) return false;
    factor++;
  }
  return true;
}