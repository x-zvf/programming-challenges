//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };
bool isTreeSymmetric(Tree<int> * t) {
    if(t == nullptr)
        return true;
    auto ltree = t->left;
    auto rtree = t->right;

    std::queue<Tree<int>*> rqueue,lqueue;
    rqueue.push(rtree);
    lqueue.push(ltree);
    while(!rqueue.empty() && !lqueue.empty()) {
        auto r = rqueue.front();
        rqueue.pop();
        auto l = lqueue.front();
        lqueue.pop();
        if(r == nullptr && l == nullptr)
            continue;
        if(r == nullptr || l == nullptr)
            return false;
        if(r->value != l->value)
            return false;
        rqueue.push(r->right);
        lqueue.push(l->left);
        rqueue.push(r->left);
        lqueue.push(l->right);
    }
    return rqueue.empty() && lqueue.empty();
}

