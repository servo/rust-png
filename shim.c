#include <stddef.h>
#include <setjmp.h>

size_t jmp_buf_size() {
  return sizeof(jmp_buf);
}
