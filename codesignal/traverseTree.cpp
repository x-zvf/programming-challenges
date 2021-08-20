//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };
vector<int> traverseTree(Tree<int> * t) {
    vector<int> ret;
    queue<Tree<int>*> q;
    q.push(t);
    while(!q.empty()){
        auto x = q.front();
        q.pop();
        if(x == nullptr)
            continue;

        ret.push_back(x->value);
        q.push(x->left);
        q.push(x->right);
    }
    return ret;
}

