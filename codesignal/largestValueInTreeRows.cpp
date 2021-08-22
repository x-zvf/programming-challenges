#include <limits>
//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };
vector<int> largestValuesInTreeRows(Tree<int> * t) {
    vector<int> res;
    queue<Tree<int>*> thisrow, nextrow;
    thisrow.push(t);
    while(!thisrow.empty()) {
        int rowmax = std::numeric_limits<int>::min();
        while(!thisrow.empty()) {
            auto x = thisrow.front();
            thisrow.pop();
            if(x == nullptr)
                continue;
            if(x->value > rowmax)
                rowmax = x->value;
            nextrow.push(x->left);
            nextrow.push(x->right);
        }
        if(rowmax > std::numeric_limits<int>::min())
            res.push_back(rowmax);
        thisrow = nextrow;
        nextrow = queue<Tree<int>*>();
    }
    return res;
}

