class Solution {
public:
    string convert(string s, int numRows) {
        if (numRows == 1) {
            return s;
        }
        
        std::vector<std::string> ret(numRows);
        
        const int n = s.length();
        bool down = true;
        for (int i = 0, tr = 0; i < n; ++i) {
            
            ret[tr].push_back(s[i]);
            
            if (down && tr == numRows - 1) {
                down = false;
                --tr;
            } else if (!down && tr == 0) {
                down = true;
                ++tr;
            } else {
                tr += down ? 1 : -1;
            }            
        }
        
        std::stringstream ss;
        for (auto& e : ret) {
            ss << e;
        }
        
        return ss.str();
    }
};
