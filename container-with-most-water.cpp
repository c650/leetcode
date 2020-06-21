
class Solution {
public:
    
    int maxArea(vector<int>& height) {
        
        int ans = go(height);
        std::reverse(height.begin(), height.end());
        ans = std::max(ans, go(height));
        
        return ans;
    }
    
    int go(std::vector<int>& v) {
        const int n = v.size();
        std::vector<std::pair<int,int>> prev;
        
        int ans{0};
        for (int i = 0; i < n; ++i) {
            if (prev.size()) {
                auto it = std::lower_bound(prev.begin(),prev.end(), std::pair<int,int>{v[i], -1});
                if (it != prev.end()) {
                    ans = std::max(ans, (i - it->second) * v[i]);
                }
            }
            
            if (prev.empty() || v[i] > prev.back().first) {
                prev.push_back({v[i], i});
            }
        }
        
        return ans;
    }
};
