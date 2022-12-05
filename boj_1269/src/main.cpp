#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

int main() {
	cin.tie(0);
	ios_base::sync_with_stdio(0);

	int n, m;
	cin >> n >> m;
	unordered_map<int, int> dict;
	vector<int> vec_a, vec_b;
	while(n--) {
		int num;
		cin >> num;
		vec_a.push_back(num);
		dict[num]++;
	}
	while(m--) {
		int num;
		cin >> num;
		vec_b.push_back(num);
		dict[num]++;
	}
	int common = vec_a.size() + vec_b.size() - dict.size();
	cout << dict.size() - common << '\n';
}
