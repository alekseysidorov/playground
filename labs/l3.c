#include <stdio.h>

int main()
{
    int a,i,k,l,n,s = 2;
    k = 1;
    l = 1;
    while (l <= 4000)
    {
        a = l + k;
        k = l;
        l = a;
        if (l % 2 == 1) s += l;
    }

    printf ("\n%d\n", s);
    return 0;
}
