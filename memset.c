#include <stdint.h>
#include <stddef.h>

void* memcpy(void* dst, const void* src, size_t size)
{
    for ( size_t i = 0; i < size; i++ )
        ((uint8_t*) dst)[i] = ((const uint8_t*) src)[i];
    return dst;
}

void* memmove(void* dst, const void* src, size_t size)
{
    for ( size_t i = 0; i < size; i++ )
        ((uint8_t*) dst)[i] = ((const uint8_t*) src)[i];
    return dst;
}


void* memset(void * s,int c,size_t count)
{
   for ( size_t i = 0; i < count; i++ )
        ((uint8_t*) s)[i] = (uint8_t)c;
    return s;
}
