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

void kernel5(int L, float* __restrict__ ai, float* __restrict__ ef) {
  int i;
  float *__restrict__ ai1 = __builtin_assume_aligned(ai,16);
  float *__restrict__ ef1 = __builtin_assume_aligned(ef,16);
#pragma GCC ivdep
//#pragma clang loop vectorize(enable) 
#pragma clang loop vectorize_width(2) interleave_count(2)
//#pragma simd
//#pragma omp simd
//#pragma unroll(8)
  for(i = 0; i < L; i++) {
    double re_part = ef1[2*i], im_part = ef1[2*i+1];
    ai1[i] += re_part * re_part + im_part * im_part;
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

  for(i = 0; i < L; i += 4) {
    __m128 ai_slice = _mm_load_ps(&ai[i]);
    __m128 re_slice = _mm_load_ps(&ef[2*i]);
    __m128 im_slice = _mm_load_ps(&ef[2*i+4]);
    re_slice = _mm_mul_ps(re_slice, re_slice);
    im_slice = _mm_mul_ps(im_slice, im_slice);
    __m128 shfl1 = _mm_shuffle_ps(re_slice, im_slice, _MM_SHUFFLE(2,0,2,0));
    __m128 shfl2 = _mm_shuffle_ps(re_slice, im_slice, _MM_SHUFFLE(3,1,3,1));
    __m128 sum = _mm_add_ps(shfl1, shfl2);
    sum = _mm_add_ps(ai_slice, sum);
    _mm_store_ps(&ai[i], sum);
  }

}
