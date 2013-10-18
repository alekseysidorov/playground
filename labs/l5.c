#include <stdio.h>

int main()
{
    int a[1000];
    int i,j,k,l,n,m;
    scanf ("%d", &n);
    k = n;
    for (i = 0; i < n; i++)
        scanf ("%d", &a[i]);
    for (i = 0; i < n * 2; i += 2)
    {
        for (j = k + 1; j > i + 1; j--)
            a[j] = a[j - 1];
        k++;
        a[i + 1] = 0;
    }
    for (i = 0; i < n * 2; i++)
        printf (" %d ", a[i]);
    return 0;
}
