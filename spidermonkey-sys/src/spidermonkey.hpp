#include <jsapi.h>

#include <js/CompilationAndEvaluation.h>
#include <js/Exception.h>
#include <js/Initialization.h>
#include <js/SourceText.h>

std::unique_ptr<JS::RealmOptions> realm_options_new();
std::unique_ptr<JS::CompileOptions> compile_options_new(JSContext* cx);
