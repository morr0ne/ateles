#include "spidermonkey.hpp"

std::unique_ptr<JS::RealmOptions> realm_options_new() {
  return std::make_unique<JS::RealmOptions>();
}

std::unique_ptr<JS::CompileOptions> compile_options_new(JSContext *cx) {
  std::unique_ptr<JS::CompileOptions> options =
      std::make_unique<JS::CompileOptions>(cx);
  options->setFileAndLine("noname", 1);

  return options;
}