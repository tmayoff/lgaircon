#ifndef HASHER_HPP
#define HASHER_HPP

#include <cstdint>

class Hasher {
  public:
    Hasher(int pin, int timeout);

    void callback(int pin, int level, uint32_t tick);

  private:
    bool inCode = false;
    int pin;
    int timeout;
    int edges;
};

#endif // HASHER_HPP
