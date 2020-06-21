class Solution {
public:
    int myAtoi(string str) {
        const auto& s = str;
        const int n = str.length();
        int start;
        for (int i = 0; i < n; ++i) {
            if (std::isspace(s[i])) {
                continue;
            }
            
            if (s[i] == '-' || s[i] == '+' || std::isdigit(s[i])) {
                start = i;
                break;
            } else {
                return 0;
            }
        }
        
        int sign = 1;
        if (s[start] == '-' || s[start] == '+') {
            if (s[start] == '-')
                sign = -1;
            ++start;
        }
        
        long ret = 0;
        
        for (int cur = start; cur < n && std::isdigit(s[cur]); ++cur) {
            ret = ret * 10;
            ret += sign * (s[cur] - '0');
            
            if (ret < std::numeric_limits<int>::min()) {
                return std::numeric_limits<int>::min();
            }
            
            if (ret > std::numeric_limits<int>::max()) {
                return std::numeric_limits<int>::max();
            }
        }
        
        return static_cast<int>(ret);
    }
};
