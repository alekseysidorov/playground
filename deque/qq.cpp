#include <iostream>
#include <string>
#include <deque>

template<typename T>
class glist
{
    struct gnode;
    struct gnode 
    {
        gnode(T d, gnode *n = 0) : data(d), next(n)
        {

        }

        T data;
        gnode * next;
    };

    gnode *head;
    size_t size_;
public:
    glist() : head(0), size_()
    {
    }

    void append (T value)
    {
        if (!head) 
        {
            head = new gnode (value, 0);
            size_++;
            return;
        }
        gnode *tmp = head;
        while (tmp->next) tmp = tmp->next;
        tmp->next = new gnode (value, 0);
        size_++;
    }

    void prepend (T value)
    {
        gnode *tmp = new gnode (value, head);
        head = tmp;
        size_++;
    }

    void insert (int i, T value)
    {
        gnode *tmp = head;
        int k = 0;
        while (tmp && k < i) 
        {
            k++;
            tmp = tmp->next;
        }

        if (tmp)
        {
            tmp->next = new gnode (value, tmp->next);
            size_++;
        }
    }


    T at (int i) const
    {
        return this->operator[] (i);
    }

    T &operator[] (int i)
    {
        gnode *tmp = head;
        int k = 0;
        while (tmp && i > k)
        {
            tmp = tmp->next;
            k++;
        }
        return tmp->data;
    }

    size_t size () const
    {
        return size_;
    }

};

bool operator < (std::string a, std::string b)
{
    if (a.size() != b.size())
        return a.size() < b.size();

    int s = a.size();
    for (int i = 0; i < s; i++)
    {
        if (a[i] != b[i]) return a[i] < b[i];
    }

    return true;
}

template<typename T>
void bubble_sort (T &list)
{
    int i = 0, j = 0;
    int s = list.size ();
    for (i = 0; i < s - 1; i++)
        for (j = 0; j < s - 1; j++)
            if (list[j] < list[j + 1])
                std::swap(list[j], list[j + 1]);
}

int main ()
{
    glist<int> sasha;
    sasha.append (117);
    sasha.prepend (222);
    sasha.insert (0, -81);
    sasha.append (98);
    sasha.insert (3, -117);
    for (int i = 0; i < sasha.size (); i++)
        std::cout << "  " << sasha[i];
    std::cout << "\n";

    std::deque<std::string> misha;
    misha.push_back("Vova");
    misha.push_front("Vodka");
    misha.push_back("Klukva");
    misha.push_front("magic");
    for (int i = 0; i < misha.size (); i++)
        std::cout << " " << misha[i];

    bubble_sort(sasha);
    
    for (int i = 0; i < sasha.size (); i++)
        std::cout << "  " << sasha[i];
    std::cout << "\n";

    std::cout << std::endl;
    return 0;
}
