#ifndef HASHER_HPP
#define HASHER_HPP

class Hasher {
  public:
    Hasher(int pin, int timeout);

  private:
    int pin;
    int timeout;
};

#endif // HASHER_HPP
