#ifndef Pair
#define Pair

#define DEFINE_PAIR(type, name)                                                \
  typedef struct {                                                             \
    type a;                                                                    \
    type b;                                                                    \
  } Pair_##name;
#endif