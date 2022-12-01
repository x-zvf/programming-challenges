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

Tree<int> *delRight(Tree<int> *t) {
    if(t->right == nullptr)
        return t->left;
    t->right = delRight(t->right);
    return t;
}

Tree<int> *dfB(Tree<int> *t, int v) {
    if(t == nullptr)
        return nullptr;
    if(v == t->value) {
        if(t->left != nullptr) {
            int m;
            auto lst = t->left;
            while(lst != nullptr && lst->right != nullptr)
                lst = lst->right;
            t->value = lst->value;
            t->left = delRight(t->left);
        } else {
            t = t->right;
        }
    } else if(v < t->value) {
        t->left = dfB(t->left, v);
    } else if(v > t->value) {
        t->right = dfB(t->right, v);
    }
    return t;
}

Tree<int> * deleteFromBST(Tree<int> * t, vector<int> queries) {
    for(int q : queries)
        t = dfB(t, q);
    return t;
}

