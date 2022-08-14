project('lgaircon', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3', 'cpp_std=c++17'])

pgpio = dependency('pigpio')

executable('lgaircon', 'main.cpp', dependencies: pgpio)
