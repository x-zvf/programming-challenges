//
// Binary trees are already defined with this interface:
// template<typename T>
// struct Tree {
//   Tree(const T &v) : value(v), left(nullptr), right(nullptr) {}
//   T value;
//   Tree *left;
//   Tree *right;
// };

Tree<int> *restore(vector<int> &inorder, vector<int> &preorder, int left, int right, int &ipre){
    if (left > right)
        return nullptr;

    Tree<int> *root = new Tree<int>(preorder[ipre++]);

    if (left == right)
        return root;

    int x;
    for (x = left; x <= right; x++){
        if (inorder[x] == root->value)
            break;
    }
    root->left = restore(inorder,preorder, left, x-1, ipre);
    root->right = restore(inorder,preorder, x + 1, right, ipre);

    return root;
}

Tree<int> * restoreBinaryTree(vector<int> inorder, vector<int> preorder) {
    int ipre = 0;
    return restore(inorder, preorder, 0, preorder.size()-1,ipre);
}
