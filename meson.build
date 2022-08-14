project('lgaircon', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3', 'cpp_std=c++17'])

cpp = meson.get_compiler('cpp')

pigpio = cpp.find_library('pigpio', required: false)

if not pigpio.found()
  cmake = import('cmake')
  pigio_proj = cmake.subproject('pigpio')
  pigpio = pigio_proj.dependency('pigpio')
endif 

executable('lgaircon', 'main.cpp', 'Hasher.cpp', dependencies: pigpio)
