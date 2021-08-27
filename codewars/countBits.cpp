unsigned int countBits(unsigned long long n){
  int count;
  for (count = 0; n; count++)
        n &= n - 1;
  return count;
}
