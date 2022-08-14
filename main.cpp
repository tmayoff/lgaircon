#include <iostream>
#include <pigpio.h>

#include "Hasher.hpp"

int main () {
  if (gpioInitialise() < 0) exit(EXIT_FAILURE);

  Hasher ir(7, 5);

  sleep(300);
}
