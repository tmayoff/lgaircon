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

void Hasher::_callback(int pin, int level, uint32_t tick) {
   if (level != PI_TIMEOUT)
   {
      if (!inCode)
      {
         inCode = true;

         gpioSetWatchdog(pin, timeout);

         hash_val = 2166136261U; /* FNV_BASIS_32 */

         edges = 1;

         t1 = 0;
         t2 = 0;
         t3 = 0;
         t4 = tick;
      }
      else
      {
         edges++;

         t1 = t2;
         t2 = t3;
         t3 = t4;
         t4 = tick;

         if (edges > 3) _hash(t2-t1, t4-t3);
      }
   }
   else
   {
      if (inCode)
      {
         inCode = false;

         gpioSetWatchdog(pin, 0);

         if (edges > 12) /* Anything less is probably noise. */
         {
            std::cout << "SIGNLA?" << std::endl;
            // (mycallback)(hash_val);
         }
      }
   }
}

void Hasher::_callback(int gpio, int level, uint32_t tick, void *user) {
   /*
      Need a static callback to link with C.
   */

   Hasher *mySelf = (Hasher *) user;

   mySelf->_callback(gpio, level, tick); /* Call the instance callback. */
}

Hasher::Hasher(int pin, int timeout) : pin(pin), timeout(timeout) {

   gpioSetMode(pin, PI_INPUT);

   gpioSetAlertFuncEx(pin, _callback, (void *)this);
}
