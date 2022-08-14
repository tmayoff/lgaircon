#include "Hasher.hpp"

#include <iostream>
#include <pigpio.h>

void Hasher::_hash(int old_val, int new_val) {
   int val;

   if      (new_val < (old_val * 0.60)) val = 13;
   else if (old_val < (new_val * 0.60)) val = 23;
   else                                 val = 2;

   hash_val ^= val;
   hash_val *= 16777619; /* FNV_PRIME_32 */
}

void Hasher::_callback(int gpio, int level, uint32_t tick) {
   if (level != PI_TIMEOUT) {
      uint32_t edge = lastTick - tick;
      lastTick = tick;

      if (fetchingCode) {
         if (edge > PreambleUS && !inCode) {
            // Start of code
            inCode = true;
            gpioSetWatchdog(gpio, timeout);
         } else if (edge > PostambleUS && inCode) {
            // End of code
            inCode = false;
            gpioSetWatchdog(gpio, 0);

            // TODO end_code
            for (uint32_t e : code) 
               std::cout << std::to_string(e);
            std::cout << std::endl;
         } else if (inCode) {
            code.push_back(edge);
         }
      }
   } else {
      gpioSetWatchdog(gpio, 0);
      if (inCode) {
         inCode = false;
      }
   }
}

void Hasher::_callback(int gpio, int level, uint32_t tick, void *user) {
   Hasher *mySelf = (Hasher *) user;
   mySelf->_callback(gpio, level, tick); /* Call the instance callback. */
}

Hasher::Hasher(int pin, int timeout) : pin(pin), timeout(timeout) {
   fetchingCode = true;
   gpioSetMode(pin, PI_INPUT);
   gpioSetAlertFuncEx(pin, _callback, (void *)this);
}
