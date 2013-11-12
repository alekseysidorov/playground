#include <iostream>
#include <stdio.h>
#include <vector>
#include <algorithm>

struct rebro_t 
{
    int v1;
    int v2;
    int size;
};

bool rebros_comparator(const rebro_t &a, const rebro_t &b)
{
    return a.size < b.size;
}

std::vector<int> p;
int dsu_get (int v) {
        return (v == p[v]) ? v : (p[v] = dsu_get (p[v]));
}
void dsu_unite (int a, int b) {
        a = dsu_get (a);
        b = dsu_get (b);
        if (rand() & 1)
            std::swap(a, b);
        if (a != b)
            p[a] = b;
}

int main ()
{
    FILE * inp, * out;
    std::vector<rebro_t> rebros;
    inp = fopen ("input.txt", "r");
    out = fopen ("output.txt", "w");
    int n, m, i, j, k, l;
    unsigned long long s = 0;
    fscanf (inp, "%d%d", &n, &m);
    rebros.reserve(m);
    rebro_t rebro;
    for (i = 0; i < m; i++)
    {
        fscanf (inp, "%d%d%d", &k, &l, &j);
        rebro.v1 = k;
        rebro.v2 = l;
        rebro.size = j;
        rebros.push_back(rebro);
    }
    std::sort(rebros.begin(), rebros.end(), rebros_comparator);
    p.reserve(n + 1);
    for (i = 0; i <= n; ++i)
        p.push_back(i);
    for (i = 0; i < m; ++i) {
        int a = rebros[i].v1, b = rebros[i].v2, l = rebros[i].size;
        if (dsu_get(a) != dsu_get(b)) {
            s += l;
            dsu_unite (a, b);
        }
    }
    
    //for (i = 0; i < m; i++)
    //{
    //    if ((!a[rebros[i].v1]) || (!a[rebros[i].v2])) 
    //    {
    //        s += rebros[i].size; 
    //        a[rebros[i].v1] = true; 
    //        a[rebros[i].v2] = true;
    //    }
    //}
    
    fprintf (out, "%llu", s);
    fclose (inp);
    fclose (out);
    return 0;
}
