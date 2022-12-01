//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };
bool hasPathWithGivenSum(Tree<int> * t, int s) {
    std::queue<std::pair<Tree<int>*,int>> q;
    q.push({t, 0});
    while(!q.empty()) {
        auto a = q.front();
        q.pop();
        if(a.first == nullptr)
            continue;
        if(a.first->left == nullptr && a.first->right == nullptr && a.second + a.first->value == s)
            return true;
        if(a.first->left != nullptr)
            q.push({a.first->left, a.second + a.first->value});
        if(a.first->right != nullptr)
            q.push({a.first->right, a.second + a.first->value});

    }
    return false;
}

