#include <iostream>
#include <cassert>
#include <cstring>

#include "ctflags.h"

/*
 * From the root directory, build the Rust library with:
 *    cargo build
 * then build the example with:
 *    g++ -o ./testclient example/testclient.cc -Iinclude -Ltarget/debug -lctflags -lpthread -ldl -lm
 */
int main() {
    ctflags::Seed seed;
    if (seed) {
        std::cout << "Seed is : " << seed.get() << std::endl;
    }

    ctflags::Flag flag("example");
    if (flag) {
        std::cout << "Example flag: " << flag.get() << std::endl;
    }

    // Test with a string context
    ctflags::Flag flag2("segg1545", "example", nullptr);
    std::cout << "Flag with string context is " << flag2.get() << std::endl;
    assert(strcmp(flag2.get(), "flag(example).5f1b958992ca66c09c0ac9170fce85de") == 0);

    return 0;
}
