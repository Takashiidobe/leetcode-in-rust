#include <array>
#include <vector>

using std::array;
using std::vector;

struct Node {
    int val = 0;
    vector<Node*> neighbors = {};
    Node() = default;
    Node(int _val) : val(_val) {}
    Node(int _val, vector<Node*> _neighbors) : val(_val), neighbors(_neighbors) {}
};

class Solution {
private:
    array<Node*, 101> v;
public:
    Node* cloneGraph(Node* node) {
       if (node == nullptr) return nullptr;

       if (v[node->val] == nullptr) {
          v[node->val] = new Node(node->val);
           for (const auto it: node->neighbors) {
               v[node->val]->neighbors.push_back(cloneGraph(it));
           }
       }

       return v[node->val];
    }
};
