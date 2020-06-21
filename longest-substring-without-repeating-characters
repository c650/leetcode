class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        const int n = s.length();
        
        int ans{0}, max_last{0};
        
        std::map<char, int> last;
        
        for (int i = 0; i < n; ++i) {            
            // end at i
            
            max_last = std::max(max_last, last[s[i]]);
            ans = std::max(ans, i + 1 - max_last);
            
            last[s[i]] = i+1;
        }
        
        return ans;
    }
};
