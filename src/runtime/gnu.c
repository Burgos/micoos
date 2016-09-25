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

int
memcmp(s1, s2, n)
    void *s1;			/* First string. */
    void *s2;			/* Second string. */
    size_t      n;                      /* Length to compare. */
{
    unsigned char u1, u2;

    for ( ; n-- ; s1++, s2++) {
	u1 = * (unsigned char *) s1;
	u2 = * (unsigned char *) s2;
	if ( u1 != u2) {
	    return (u1-u2);
	}
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

