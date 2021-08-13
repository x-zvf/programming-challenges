vector<vector<string>> groupingDishes(vector<vector<string>> dishes) {
    std::map<string, std::set<std::string>> ingr_d;
    for(const auto &dish : dishes) {
        std::string dish_name = dish[0];
        for(int i=1; i<dish.size(); i++) {
                   ingr_d[dish[i]].insert(dish_name);
        }
    }
    vector<vector<string>> result;
    for(const auto &[ingr, c_d] : ingr_d){
        if(c_d.size() < 2)
            continue;
        vector<string> r;
        r.push_back(ingr);
        for(auto &d : c_d)
            r.push_back(d);
        result.push_back(r);
    }
    return result;
}

