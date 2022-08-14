#include "Hasher.hpp"

#include <iostream>
#include <pigpio.h>

void callbackFunc(int pin, int level, uint32_t tick, void *user) {

    Hasher *self = (Hasher*) user;
    user->_callback(pin, level, tick);
}

Hasher::Hasher(int pin, int timeout): pin(pin), timeout(timeout) {
    gpioSetMode(pin, PI_INPUT);
    gpioSetAlertFuncEx(pin, callbackFunc, (void*)this);
}


void Hasher::callback(int pin, int level, uint32_t tick) {
    if (level != PI_TIMEOUT) {
        if (!inCode) {
            inCode = true;

            gpioSetWatchdog(pin, timeout);

            edges = 1;
        } else {
            edges++;
        }
    } else {
        if (inCode) {
            inCode = false;

            gpioSetWatchdog(pin, 0);
            if (edges > 12) {
                std::cout << "SIGNAL?" << std::endl;
            }
        }
    }
}