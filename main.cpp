#include <iostream>
#include <pigpio.h>

int main () {
  if (gpioInitialize() >= 0) {
   std::cout << "GPIO initialized" << std::endl; 
  }
}
