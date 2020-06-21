class Solution {
public:
    string minWindow(string s, string t) {
        
        std::map<char, int> f;
        std::map<char, int> needs;
        for (auto& e : t) {
            ++needs[e];
        }
        
        int need = t.length();
        
        const int n = s.length();
        
        int aj = n + 2;
        int ai = 0;
        
        for (int i = 0, j = 0; j < n; ++j) {
            if (++f[s[j]] <= needs[s[j]]) {
                --need;
            }
            
            while (i < j && f[s[i]] > needs[s[i]]) {
                --f[s[i++]];
            }
            
            if (need == 0 && aj - ai > j - i) {
                aj = j;
                ai = i;
            }
        }
        
        
        if (aj - ai > n)
            return "";

        return s.substr(ai, aj - ai + 1);        
    }
};
