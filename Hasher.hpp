#ifndef HASHER_HPP
#define HASHER_HPP

#include <cstdint>
#include <string>

class Hasher {
  enum class Level {Falling = 0, Rising = 1, NoChange = 2};
  static const std::string GetLevelStr(Level l) {
    switch (l) {
    case Level::Falling:
      return "Falling";
    case Level::Rising:
      return "Rising";
    case Level::NoChange:
      return "No Change";
    default:
      return "Unknown";
    } 
  }

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
