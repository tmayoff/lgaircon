#include <iostream>
#include <unistd.h>
#include <lirc_client.h>

int main () {
  const char* lircrc_path;
  lirc_config* config = nullptr;

  if (lirc_init("lgaircon", 1) == -1) {
    std::cout << "Failed to initialize" << std::endl;
    return EXIT_FAILURE;
  }

  if (lirc_readconfig(nullptr, &config, nullptr) != 0) {
    std::cout << "Failed to read config" << std::endl;
    return EXIT_FAILURE;
  }


  char* code;
  char* c;
  while (lirc_nextcode(&code) == 0) {
    printf("Code: %s\n", code);
    int ret = 0;
    while((ret = lirc_code2char(config, code, &c))) {
      printf("Command: %s", c);
    }

    free(code);
    if (ret == -1) break;
  }

  lirc_freeconfig(config);
  lirc_deinit();
}
