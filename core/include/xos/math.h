#ifndef __MATH_H__
#define __MATH_H__

#ifndef __cplusplus
#define min(a, b) ((a) < (b) ? (a) : (b))
#define max(a, b) ((a) < (b) ? (b) : (a))
#endif

#define flooru(v, m) ((v) - ((v) % (m)))
#define ceilu(v, m) (((v) % (m) > 0) ? flooru((v) + (m), (m)) : (v))

#endif
