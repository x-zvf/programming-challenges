//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };

std::pair<bool,int> x(Tree<int> *t, int &k) {
    if(t == nullptr)
    return {false,0};

    auto l = x(t->left, k);
    if(l.first)
        return l;

    if(--k == 0)
        return {true,t->value};
    return x(t->right, k);
}

int kthSmallestInBST(Tree<int> * t, int k) {
    return x(t,k).second;
}

