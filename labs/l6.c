#include <stdio.h>
#include <string.h>

int main()
{
    FILE *f1, *f2;
    f1 = fopen ("input.txt", "r");
    f2 = fopen ("output.txt", "w");
    char s[10000];
    fgets (s, sizeof (s), f1);
    fscanf (f1,"%s", s);
    int i,k,l,n,m;
    k = 0;
    i = 0;
    while (strlen (s))
    {
            l = 1;
            while (s[i + 1] == s[i])
            {
                i++;
                l++;
            }
            if (l > 1) fprintf(f2, "%d%c", l, s[i - 1]);
            else { fprintf (f2, "%d%c", l, s[i]); i++; }
    }
    return 0;
}
