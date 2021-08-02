#include <iostream>
#include <string>

using namespace std;

int solution(string roman) {
  int sum = 0;
  std::map<char,int> digitval = {
    {'I',1}, {'V', 5}, {'X',10},
    {'L', 50}, {'C', 100}, {'D', 500}, {'M',1000}};
  for(int i = 0; i<roman.length(); i++) {
      int cv = digitval[roman[i]];
      if(i < (roman.length()-1)) {
        int nv = digitval[roman[i+1]];
        if(cv >= nv){
          sum += cv;
        } else {
          sum += nv;
          sum -= cv;
          i++;
        }
      } else {
        sum += cv;
      }
  }
  return sum;
}