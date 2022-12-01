#include<bits/stdc++.h>
class RGBToHex
{
  public:
  static std::string rgb(int r, int g, int b){
    std::stringstream ss;
    for(int v : {r,g,b}) {
      v = std::clamp(v, 0, 255);
      ss << std::hex << std::uppercase << std::setfill('0') << std::setw(2) << v;
    }
    return ss.str();
  }
};
