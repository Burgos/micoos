////epub unsafe extern fn __aeabi_memclr(s: *mut u8, n: usize) -> *mut u8

char* __aeabi_memclr(char* s, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(s + i) = 0;
    }
}

char* __aeabi_memclr8(char* s, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(s + i) = 0;
    }
}

char* __aeabi_memclr4(char* s, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(s + i) = 0;
    }
}

void __aeabi_memcpy8(char* dest, char* src, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(dest + i) = *(src + i);
    }
}

void __aeabi_memcpy4(char* dest, char* src, unsigned int n) 
{
    for (unsigned int i = 0; i < n; i++)
    {
        *(dest + i) = *(src + i);
    }
}

