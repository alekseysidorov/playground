#include <iostream>

constexpr int fsum(int a, int b, int sum)
{
    return b <= 4000 ? fsum(b, a + b, b % 2 ? (sum += b) : sum) : sum;  
}

int main()
{
    auto s = fsum(1, 1, 2);

    std::cout << s;
    return 0;
}
