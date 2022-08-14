#include "Hasher.hpp"

#include <iostream>
#include <pigpio.h>

void callbackFunc(int pin, int level, uint32_t tick, void *user) {
    std::cout << "Received" << std::endl;
}

Hasher::Hasher(int pin, int timeout): pin(pin), timeout(timeout) {
    gpioSetMode(pin, PI_INPUT);
    gpioSetAlertFunctionEx(pin, callbackFunc, (void*)this);
}
