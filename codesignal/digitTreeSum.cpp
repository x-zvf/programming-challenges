//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };
long long digitTreeSum(Tree<int> * t) {
    stack<std::pair<Tree<int>*,vector<int>>> stk;
    stk.push({t,{0}});
    long long sum = 0;
    while(!stk.empty()) {
        auto p = stk.top();
        stk.pop();
        if(p.first == nullptr)
            continue;
        auto arr = p.second;
        arr.push_back(p.first->value);
        if(p.first->left == nullptr && p.first->right == nullptr) {
            long long s = 0;
            long long times = 1;
            for(auto it = arr.rbegin(); it != arr.rend(); it++) {
                 s += *it * times;
                 times *= 10;
            }
            sum += s;
        } else {
            stk.push({p.first->left, arr});
            stk.push({p.first->right, arr});
        }
    }
    return sum;
}

