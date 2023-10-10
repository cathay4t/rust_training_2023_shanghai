// SPDX-License-Identifier: Apache-2.0

#include <iostream>
#include <vector>

int main() {
    std::vector<int> vector1 = {1};
    int *first = &vector1[0];
    int i = 0;

    std::cout << "Before: " << *first << "\n";

    // ranged loop
    for (; i < 100; ++i) {
        vector1.push_back(i);
    }
    std::cout << "After:  " << *first << "\n";
}
