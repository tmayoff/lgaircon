#ifndef HASHER_HPP
#define HASHER_HPP

#include <cstdint>

class Hasher {
  public:
    Hasher(int pin, int timeout);

    static void _callback(int gpio, int level, uint32_t tick, void *user);
    void _callback(int pin, int level, uint32_t tick);

    void _hash(int old_val, int new_val);

  private:
    bool inCode = false;
    int pin;
    int timeout;
    int edges;
    uint32_t hash_val;
    uint32_t t1, t2, t3, t4;
};

#endif // HASHER_HPP
