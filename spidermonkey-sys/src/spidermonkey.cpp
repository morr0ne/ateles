#include "spidermonkey.hpp"

std::unique_ptr<JS::RealmOptions> realm_options_new() {
  return std::make_unique<JS::RealmOptions>();
}
