#include <iostream>
#include <unistd.h>
#include <pigpio.h>

#include "Hasher.hpp"

int main () {
  std::cout << "Initializing..." << std::endl;
  if (gpioInitialise() < 0) exit(EXIT_FAILURE);
  std::cout << "Initialized." << std::endl;

  Hasher ir(19, 5);

  sleep(300);
}
