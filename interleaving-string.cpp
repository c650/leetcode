class Solution {
public:
    bool isInterleave(string s1, string s2, string s3) {
        if (s1.length() + s2.length() != s3.length()) {
            return false;
        }
        
        std::vector<std::vector<char>> memo(1 + s1.length(), std::vector<char>(1 + s2.length(), -1));
        
        std::function<bool(int,int)> go = [&](int a, int b) -> bool {
            
            int c = a + b;
            if (c >= static_cast<int>(s3.length())) {
                return true;
            }
            
            char& ret = memo[a][b];
            if (ret != -1) {
                return ret;
            }
            
            ret = false;
            
            ret |= (a < static_cast<int>(s1.length()) && s3[c] == s1[a] && go(a+1, b));
            ret |= (b < static_cast<int>(s2.length()) && s3[c] == s2[b] && go(a, b+1));
                       
            return ret;
        };
        
        return go(0,0);
    }
};
