#include <string>

std::string format_duration(int seconds) {
    if(seconds == 0)
        return "now";
    int minutes = 0, hours = 0, days = 0, years=0;
    while(seconds >= 60*60*24*365) {
        seconds -= 60*60*24*365;
        years += 1;
    }
    while(seconds >= 60*60*24) {
        seconds -= 60*60*24;
        days += 1;
    }
    while(seconds >= 60*60) {
        seconds -= 60*60;
        hours += 1;
    }
    while(seconds >= 60) {
        seconds -= 60;
        minutes += 1;
    }
    std::vector<std::string> components;
    if(years > 0){
      std::stringstream ss;
      ss << years << " year" << (years > 1 ? "s" : "");
      components.push_back(ss.str());
    }
    if(days > 0){
      std::stringstream ss;
      ss << days << " day" << (days > 1 ? "s" : "");
      components.push_back(ss.str());
    }
    if(hours > 0){
      std::stringstream ss;
      ss << hours << " hour" << (hours > 1 ? "s" : "");
      components.push_back(ss.str());
    }
    if(minutes > 0){
      std::stringstream ss;
      ss << minutes << " minute" << (minutes > 1 ? "s" : "");
      components.push_back(ss.str());
    }
    if(seconds > 0){
      std::stringstream ss;
      ss << seconds << " second" << (seconds > 1 ? "s" : "");
      components.push_back(ss.str());
    }
    std::stringstream res;
    res << components[0];
    for(int i=1; i<(components.size()-1); i++)
        res << ", " << components[i];
    if(components.size() > 1)
        res << " and " << components[components.size()-1];
    return res.str();
}
