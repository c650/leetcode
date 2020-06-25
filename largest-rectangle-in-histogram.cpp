class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        
        if (heights.empty()) {
            return 0;
        }
        
        std::vector<std::pair<int,int>> vec;
        std::transform(heights.begin(),heights.end(), std::back_inserter(vec), [](int a) {return std::make_pair(a,-a);});
        
        go(vec);
        std::reverse(vec.begin(),vec.end());
        go(vec);
        
        return std::max_element(vec.begin(), vec.end(), [](auto a, auto b){return a.second < b.second;})->second;
    }
    
    void go(std::vector<std::pair<int,int>>& vec) {
        std::stack<std::pair<int,int>> stk;
        stk.push(std::make_pair(-1,-1));
        
        const int n = vec.size();
        
        for (int i = 0; i < n; ++i) {
            while (stk.top().first >= vec[i].first) {
                stk.pop();
            }
            
            assert(stk.size());
            
            
            vec[i].second += vec[i].first * (i - stk.top().second);
            stk.push({vec[i].first, i});
        }
    }
};
