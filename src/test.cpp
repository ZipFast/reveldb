#include <vector>
#include <algorithm>
#include <string>
#include <unordered_set>
using namespace std;
class Solution
{
public:
	int n, m;
	int res = INT_MAX;
	int dfs(vector<vector<int>> &mat, int i, int target)
	{
		if (i == n)
		{
			return abs(target);
		}
		for (int j = 0; j < n; j++)
		{
			int t = dfs(mat, i + 1, target - mat[i][j]);
			res = min(res, t);
			return res;
		}
	}
	int minimizeTheDifference(vector<vector<int>> &mat, int target)
	{
		m = mat.size();
		n = mat[0].size();
		for (auto &v : mat)
		{
			sort(v.begin(), v.end());
		}
		return dfs(mat, 0, target);
	}
};