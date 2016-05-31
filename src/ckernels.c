
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
