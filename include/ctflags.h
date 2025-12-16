#ifndef __CTFLAGS_H_
#define __CTFLAGS_H_

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#include <utility> // Pour std::move

extern "C" {
    const char* ctflags_get_seed_or_null();
    void ctflags_free_string(const char* ptr);
    const char* ctflags_format_flag(const char* step, const char* salt);
    const char* ctflags_format_flag_from_string_context(const char* context, const char* step, const char* salt);
}

namespace ctflags {

class Seed {
private:
    const char* m_ptr = nullptr;

public:
    Seed() : m_ptr(ctflags_get_seed_or_null()) {}

    ~Seed() {
        ctflags_free_string(m_ptr);
    }

    // Copy forbidden to avoid double free
    Seed(const Seed&) = delete;
    Seed& operator=(const Seed&) = delete;

    Seed(Seed&& other) noexcept : m_ptr(other.m_ptr) {
        other.m_ptr = nullptr;
    }
    Seed& operator=(Seed&& other) noexcept {
        if (this != &other) {
            ctflags_free_string(m_ptr);
            m_ptr = other.m_ptr;
            other.m_ptr = nullptr;
        }
        return *this;
    }

    const char* get() const {
        return m_ptr;
    }

    explicit operator bool() const {
        return m_ptr != nullptr;
    }
};


class Flag {
private:
    const char* m_ptr = nullptr;

public:
    Flag(const char* step, const char* salt = nullptr)
        : m_ptr(ctflags_format_flag(step, salt)) {}

    Flag(const char* context, const char* step, const char* salt = nullptr)
        : m_ptr(ctflags_format_flag_from_string_context(context, step, salt)) {}

    ~Flag() {
        ctflags_free_string(m_ptr);
    }

    // Copy forbidden to avoid double free
    Flag(const Flag&) = delete;
    Flag& operator=(const Flag&) = delete;

    Flag(Flag&& other) noexcept : m_ptr(other.m_ptr) {
    }
    Flag& operator=(Flag&& other) noexcept {
        if (this != &other) {
            ctflags_free_string(m_ptr);

            m_ptr = other.m_ptr;
            other.m_ptr = nullptr;
        }
        return *this;
    }

    const char* get() const {
        return m_ptr;
    }
    explicit operator bool() const {
        return m_ptr != nullptr;
    }
};

} // namespace ctflags
#endif  // __CTFLAGS_H_
