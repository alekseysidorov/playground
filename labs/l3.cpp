#include <iostream>

constexpr int fsum(const int a, const int b, const int sum)
{
    return a <= 4000 ? fsum(b, a + b, a  % 2 ? sum + a : sum) 
        : sum;  
}

int main()
{
    auto s = fsum(0, 1, 0);

    std::cout << s;
    return 0;
}
