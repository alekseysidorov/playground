#include <stdio.h>

int main()
{
    FILE *f1, *f2;
    f1 = fopen ("input.txt", "r");
    f2 = fopen ("output.txt", "w");
    int k,s = 0;
    while (1)
    {
        fscanf (f1, "%d", &k);
        if (! feof (f1))
            switch (k)
            {
                case 1: 
                    s += 50;
                    break;
                case 2: 
                    s += 60;
                    break;
                case 3:
                    s += 100;
                    break;
                case 4:
                    s += 30;
                    break;
            }
        else break;
    }
    fprintf (f2, "%d", s);
    return 0;
}
