#include <stdio.h>

int main()
{
    FILE *f1, *f2;
    f1 = fopen ("input.txt", "r");
    f2 = fopen ("output.txt", "w");
    int n,a = 0;
    long long s = 0;
    fscanf (f1, "%d", &n);
    for (int i = 0; i < n; i++)
    {
        fscanf (f1, "%d", &a);
        s += a*a;
    }
    fprintf (f2, "\n%lld\n", s);
    return 0;
}
