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
  for(i = 0; i < L; i++) {
    ai[i] += (double)ef[2*i] * (double)ef[2*i] + (double)ef[2*i+1] * (double)ef[2*i+1];
  } 
}

void kernel6(int L, float* ai, float* ef) {
  int i;
  for(i = 0; i < L/4; i++) {
    ai[i + 3] += (double)ef[2*(i + 3)] * (double)ef[2*(i + 3)] + (double)ef[2*(i + 3)+1] * (double)ef[2*(i + 3)+1];
    ai[i + 2] += (double)ef[2*(i + 2)] * (double)ef[2*(i + 2)] + (double)ef[2*(i + 2)+1] * (double)ef[2*(i + 2)+1];
    ai[i + 1] += (double)ef[2*(i + 1)] * (double)ef[2*(i + 1)] + (double)ef[2*(i + 1)+1] * (double)ef[2*(i + 1)+1];
    ai[i + 0] += (double)ef[2*(i + 0)] * (double)ef[2*(i + 0)] + (double)ef[2*(i + 0)+1] * (double)ef[2*(i + 0)+1];
  } 
}
