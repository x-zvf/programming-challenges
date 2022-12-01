#include <limits>
class Node {
public:
    Node(int l,int d, bool f, Node* parent)
        : length(l), depth(d), file(f), parent(parent) {}
    int length;
    int depth;
    bool file;
    Node* parent;
    vector<Node*> children;

    void print() {
        for(int i=0; i<depth; i++)
            std::cout << "\t";
        std::cout<<"len=" << length << " file=" << file <<"\n";
        for(auto &child : children)
            child->print();
    }
};

int longestPath(string fileSystem) {
    Node *root = new Node(0,-1,false,nullptr);

    Node *cnode = root;
    int cwlength = 0;
    int cdepth = 0;
    bool cwfile = false;
    for(char &c : fileSystem) {
        if(c == '\f') {
            //std::cout << "hit formfeed: l=" << cwlength << "d=" << cdepth << "  f=" << cwfile << "\n";
            while(cnode->depth >= cdepth) {
                cnode = cnode->parent;
            }
            auto nn = new Node(cwlength, cdepth, cwfile, cnode);
            cnode->children.push_back(nn);
            cnode = nn;
            cwlength = 0;
            cwfile = false;
            cdepth = 0;
        } else if (c == '\t') {
            cdepth++;
        } else if (c == '.') {
            cwfile = true;
            cwlength++;
        } else {
            cwlength++;
        }
    }
    while(cnode->depth >= cdepth) {
        cnode = cnode->parent;
    }
    auto nn = new Node(cwlength, cdepth, cwfile, cnode);
    cnode->children.push_back(nn);

    //std::cout << "tree:\n";
    //root->print();

    int maxl = std::numeric_limits<int>::min();

    stack<pair<Node*, int>> stk;
    stk.push({root,0});

    while(!stk.empty()) {
        auto x = stk.top();
        stk.pop();
        if(x.first->children.size() == 0) {
            if(x.first->file) {
                int len = x.second + x.first->length - 1;
                if(len > maxl)
                    maxl = len;
            }
        } else {
            for(auto *child : x.first->children) {
                stk.push({child, x.second + x.first->length + 1});
            }
        }
    }

    return maxl == std::numeric_limits<int>::min() ? 0 : maxl;
}

