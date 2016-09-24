// Contains runtime functions needed to link,
// as it the kernel image is not linked against
// libruntime
#include <stddef.h>

char* memclr(char* s, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(s + i) = 0;
    }

    return s;
}

char* memset (char* s, int c, size_t n)
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(s + i) = c;
    }

    return s;
}

int memcmp (char* s1, char* s2, size_t n)
{
    for (int i = 0; i < n; i++)
    {
        unsigned char diff = *(s1 + i) - *(s2 + i);

        if (diff == 0)
        {
                continue;
        }

        return diff > 0? 1 : -1;
    }

    return 0;
}

void memcpy(char* dest, char* src, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(dest + i) = *(src + i);
    }
}
