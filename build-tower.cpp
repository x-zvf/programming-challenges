6 kyu
Build Tower
C++:

class Kata
{
public:
    std::vector<std::string> towerBuilder(int nFloors)
    {
        std::vector<std::string> tower;
        int len = (nFloors * 2) - 1;
        for(int floor = 1; floor <= nFloors; floor++){
          int padding = nFloors - floor;
          std::stringstream ss;
          for(int i = 0; i < padding; i++)
            ss<<' ';
          for(int i = 0; i < 1+(2*(floor-1)); i++)
            ss<<'*';
          for(int i = 0; i < padding; i++)
            ss<<' ';
          tower.push_back(ss.str());
        }
        return tower;
    }
};