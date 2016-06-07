#include <emmintrin.h>

void kernel4(int L, float* ai, float* ef) {
  float* p2 = ef;
  float* p1 = ai;
  double   mag, t;
  int i;
  for(i = 0; i < L; i++) {
    t = *p2++;
    mag = t*t;
    t = *p2++;
    mag += t*t;   
    *p1++ += mag;
  } 
}

void kernel5(int L, float* ai, float* ef) {
  int i;
//#pragma GCC ivdep
//#pragma unroll(8)
  for(i = 0; i < L; i++) {
//    ai[i] += (double)ef[2*i] * (double)ef[2*i] + (double)ef[2*i+1] * (double)ef[2*i+1];
    double re_part = ef[2*i], im_part = ef[2*i+1];
    ai[i] += re_part * re_part + im_part * im_part;
  } 
}

void kernel6(int L, float* ai, float* ef) {
  int i;
  for(i = 0; i < L; i += 4) {
    ai[i + 0] += (double)ef[2*(i + 0)] * (double)ef[2*(i + 0)] + (double)ef[2*(i + 0)+1] * (double)ef[2*(i + 0)+1];
    ai[i + 1] += (double)ef[2*(i + 1)] * (double)ef[2*(i + 1)] + (double)ef[2*(i + 1)+1] * (double)ef[2*(i + 1)+1];
    ai[i + 2] += (double)ef[2*(i + 2)] * (double)ef[2*(i + 2)] + (double)ef[2*(i + 2)+1] * (double)ef[2*(i + 2)+1];
    ai[i + 3] += (double)ef[2*(i + 3)] * (double)ef[2*(i + 3)] + (double)ef[2*(i + 3)+1] * (double)ef[2*(i + 3)+1];
  } 
}

void kernel7(int L, float* ai, float* ef) {
  int i;
//  for(i = 0; i < L; i++) {
//    //__builtin_prefetch(ef + 2*i + L);
//    //__builtin_prefetch(ai + i + L);
//    double re_part = ef[2*i], im_part = ef[2*i+1];
//    ai[i] += re_part * re_part + im_part * im_part;
//  }
  for(i = 0; i < L; i += 8) {
    ai[i + 0] += (double)ef[2*(i + 0)] * (double)ef[2*(i + 0)] + (double)ef[2*(i + 0)+1] * (double)ef[2*(i + 0)+1];
    ai[i + 1] += (double)ef[2*(i + 1)] * (double)ef[2*(i + 1)] + (double)ef[2*(i + 1)+1] * (double)ef[2*(i + 1)+1];
    ai[i + 2] += (double)ef[2*(i + 2)] * (double)ef[2*(i + 2)] + (double)ef[2*(i + 2)+1] * (double)ef[2*(i + 2)+1];
    ai[i + 3] += (double)ef[2*(i + 3)] * (double)ef[2*(i + 3)] + (double)ef[2*(i + 3)+1] * (double)ef[2*(i + 3)+1];

    ai[i + 0 + 4] += (double)ef[2*(i + 0 + 4)] * (double)ef[2*(i + 0 + 4)] + (double)ef[2*(i + 0 + 4)+1] * (double)ef[2*(i + 0 + 4)+1];
    ai[i + 1 + 4] += (double)ef[2*(i + 1 + 4)] * (double)ef[2*(i + 1 + 4)] + (double)ef[2*(i + 1 + 4)+1] * (double)ef[2*(i + 1 + 4)+1];
    ai[i + 2 + 4] += (double)ef[2*(i + 2 + 4)] * (double)ef[2*(i + 2 + 4)] + (double)ef[2*(i + 2 + 4)+1] * (double)ef[2*(i + 2 + 4)+1];
    ai[i + 3 + 4] += (double)ef[2*(i + 3 + 4)] * (double)ef[2*(i + 3 + 4)] + (double)ef[2*(i + 3 + 4)+1] * (double)ef[2*(i + 3 + 4)+1];

  }
}
