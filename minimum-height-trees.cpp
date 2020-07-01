class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        std::vector<std::vector<int>> gr(n);
        for (auto& e : edges) {
            gr[e.front()].push_back(e.back());
            gr[e.back()].push_back(e.front());
        }
        
        std::vector<int> dist(n, n+2);
        dist[0] = 0;
        std::queue<int> ord;
        ord.push(0);
        
        while (ord.size()) {
            auto cur = ord.front();
            ord.pop();
            
            for (const auto& nei : gr[cur]) {
                if (dist[nei] > 1 + dist[cur]) {
                    dist[nei] = 1 + dist[cur];
                    ord.push(nei);
                }
            }
        }
        
        auto it = std::max_element(dist.begin(),dist.end());
        int node = std::distance(dist.begin(), it);
        
        dist.assign(n, n+2);
        dist[node] = 0;
        
        ord.push(node);
        
        std::vector<int> p(n,-1);
        
        while (ord.size()) {
            auto cur = ord.front();
            ord.pop();
            
            for (const auto& nei : gr[cur]) {
                if (dist[nei] > 1 + dist[cur]) {
                    dist[nei] = 1 + dist[cur];
                    p[nei] = cur;
                    ord.push(nei);
                }
            }
        }
        
        it = std::max_element(dist.begin(),dist.end());
        int node2 = std::distance(dist.begin(), it);
        
        // node and node2 are two leaves that make up endpoints of diameter
        
        std::vector<int> me(n, -1);
        
        std::vector<char> ondiam(n,false);
        for (int cur = node2; cur != -1; cur = p[cur]) {
            ondiam[cur] = true;
        }
        
        std::function<void(int,int)> go = [&](int cur, int prev) {
            for (const auto& nei : gr[cur]) {
                if (nei == prev) continue;
                if (ondiam[nei]) continue;
                
                me[nei] = me[cur] + 1;
                go(nei, cur);
            }
        };
        
        int cur = node2, dnode2 = 0, dnode = dist[node2];
        while (cur != -1) {
            me[cur] = std::max(dnode2, dnode);
            
            go(cur, -1);
            
            dnode2++;
            dnode--;
            cur = p[cur];
        }
        
        int min = *std::min_element(me.begin(),me.end());
        
        std::vector<int> ans;
        for (int i = 0; i < n; ++i) {
            if (me[i] == min)
                ans.push_back(i);
        }
        
        return ans;
    }
};
