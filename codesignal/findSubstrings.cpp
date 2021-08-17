class TrieNode {
public:
    TrieNode(char x) : c(x) {}
    char c;
    bool end = false;
    std::unordered_map<char, TrieNode*> children;
};

vector<string> findSubstrings(vector<string> words, vector<string> parts) {
    auto trie = new TrieNode('\0');
    for(auto &part : parts){
        TrieNode *node = trie;
        for(char l : part) {
            if(!node->children.count(l))
                node->children[l] = new TrieNode(l);
            node = node->children[l];
        }
        node->end = true;
    }

    for(auto &word : words) {
        int maxlen = -1;
        int maxpos = -1;
        for(int start = 0; start<word.length(); start++) {
            auto node = trie;
            for(int p = start; p<word.length(); p++) {
                char c = word[p];
                if(!node->children.count(c))
                    break;

                int len = 1 + p - start;
                node = node->children[c];
                if(node->end && len > maxlen) {
                    maxlen = len;
                    maxpos = start;
                }
            }
        }
        if(maxlen > 0) {
            stringstream ss;
            int i = 0;
            for(; i < maxpos; i++)
                ss << word[i];
            ss << '[';
            for(; i < maxpos+maxlen; i++)
                ss << word[i];
            ss << ']';
            for(; i < word.length(); i++)
                ss << word[i];
            word = ss.str();
        }
    }

    return words;
}

