#include <vector>

PeakData pick_peaks(std::vector<int> v) {
  PeakData result;
  if(v.size() == 0) return result;
  bool plat = false;
  int pp = -1;
  for(int i = 1; i<v.size()-1; i++) {
    if(v[i] > v[i-1] && v[i] > v[i+1]){
      result.pos.push_back(i);
      result.peaks.push_back(v[i]);
    } else if(v[i] > v[i-1] && v[i] == v[i+1]) {
      plat = true;
      pp = i;
    } else if(plat && v[i] > v[i+1]) {
      plat = false;
      result.pos.push_back(pp);
      result.peaks.push_back(v[pp]);
    } 
    if(plat && v[i] < v[i+1]) {
      plat = false;
    }
  }
  return result;
}