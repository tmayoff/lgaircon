#ifndef HASHER_HPP
#define HASHER_HPP

#include <cstdint>
#include <vector>
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

  const uint32_t PreambleMS = 200;
  const uint32_t PreambleUS = PreambleMS * 1000;

  const uint32_t PostambleMS = 15;
  const uint32_t PostambleUS = PostambleMS * 1000;

  const uint32_t ShortCodeLenge = 10;

  public:
    Hasher(int pin, int timeout);

    static void _callback(int gpio, int level, uint32_t tick, void *user);
    void _callback(int pin, int level, uint32_t tick);

    void _hash(int old_val, int new_val);

  private:
    std::vector<uint32_t> code;
    uint32_t lastTick = INT32_MAX;
    bool fetchingCode = false;
    bool inCode = false;
    int pin;
    int timeout;
    int edges;
    uint32_t hash_val;
    uint32_t t1, t2, t3, t4;
};

#endif // HASHER_HPP
