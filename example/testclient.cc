#include <iostream>

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

    return 0;
}
