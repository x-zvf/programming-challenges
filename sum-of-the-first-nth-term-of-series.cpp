#include<iostream>
#include<sstream>
#include<iomanip>
std::string seriesSum(int n)
{
    if(n < 1) return "0.00";
    double sum = 1.0;
    for(int i = 0; i < n-1; i++)
      sum += 1.0/(4+(3*i));
    std::stringstream ss;
    ss << std::fixed << std::setprecision(2) << sum;
    return ss.str();
}