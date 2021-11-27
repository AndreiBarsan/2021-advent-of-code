#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

const long TARGET = 2020L;

long solve_a(const std::vector<long> &entries) {
    for (int i = 0; i < entries.size() - 1; ++i) {
        for (int j = i + 1; j < entries.size(); ++j) {
            if (entries[i] + entries[j] == TARGET)
            {
                return entries[i] * entries[j];
            }
        }
    }
    return -1;
}

long solve_b(const std::vector<long> &entries) {
    for (int i = 0; i < entries.size() - 2; ++i) {
        for (int j = i + 1; j < entries.size() - 1; ++j) {
            long sub = entries[i] + entries[j];
            if (sub > TARGET) {
                continue;
            }
            for (int k = j + 1; k < entries.size(); ++k)
            {
                if (sub + entries[k] == TARGET)
                {
                    return entries[i] * entries[j] * entries[k];
                }
            }
        }
    }
    return -1;
}

int main() {
    ifstream in("001-input.txt");
    long val = -1;
    vector<long> entries;
    while (in >> val) {
        entries.push_back(val);
    }

    cout << solve_b(entries) << "\n";

    return 0;
}