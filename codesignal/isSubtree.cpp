//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };
bool treeEqual(Tree<int> * t1, Tree<int> * t2) {
    queue<Tree<int>*> a,b;
    a.push(t1);
    b.push(t2);
    while(!a.empty() && !b.empty()) {
        auto x = a.front();
        a.pop();
        auto y = b.front();
        b.pop();
        if(x == nullptr && y == nullptr)
            continue;
        if(x == nullptr || y == nullptr)
            return false;
        if(x->value != y->value)
            return false;
        a.push(x->left);
        a.push(x->right);
        b.push(y->left);
        b.push(y->right);
    }
    return a.empty() && b.empty();
}

bool isSubtree(Tree<int> * t1, Tree<int> * t2) {
    if(t2 == nullptr)
        return true;
    queue<Tree<int>*> q;
    q.push(t1);
    while(!q.empty()) {
        auto c = q.front();
        q.pop();
        if(c == nullptr)
            continue;
        if(c->value == t2->value && treeEqual(c, t2))
            return true;
        q.push(c->left);
        q.push(c->right);
    }
    return false;
}

