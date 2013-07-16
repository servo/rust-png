#include <setjmp.h>
#include <png.h>

jmp_buf *pngshim_jmpbuf(png_struct *png_ptr) {
  return &png_jmpbuf(png_ptr);
}
