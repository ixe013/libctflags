#ifndef __CTFLAGS_H_
#define __CTFLAGS_H_

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#include <utility> // Pour std::move

// --- Étape 1: Déclarer les fonctions "C" brutes exportées par Rust ---
// Cette partie est la seule qui doit rester synchronisée avec la librairie Rust.
extern "C" {
    const char* ctflags_get_seed_or_null();
    void ctflags_free_string(const char* ptr);
    const char* ctflags_format_flag(const char* step, const char* salt);
}

// --- Étape 2: Créer une classe C++ qui gère la ressource ---
// Cette classe offre une interface sûre et automatique.
namespace ctflags {

class Seed {
private:
    const char* m_ptr = nullptr;

public:
    // Le constructeur acquiert la ressource.
    Seed() : m_ptr(ctflags_get_seed_or_null()) {}

    // Le destructeur libère la ressource.
    ~Seed() {
        ctflags_free_string(m_ptr);
    }

    // --- Gestion de la propriété exclusive ---
    // On interdit la copie pour éviter les double-libérations.
    Seed(const Seed&) = delete;
    Seed& operator=(const Seed&) = delete;

    // On autorise le "move" pour permettre le transfert de propriété.
    Seed(Seed&& other) noexcept : m_ptr(other.m_ptr) {
        other.m_ptr = nullptr; // L'ancien objet ne possède plus le pointeur.
    }
    Seed& operator=(Seed&& other) noexcept {
        if (this != &other) {
            ctflags_free_string(m_ptr); // Libère notre propre ressource
            m_ptr = other.m_ptr;        // Vole le pointeur de l'autre
            other.m_ptr = nullptr;      // L'autre ne le possède plus
        }
        return *this;
    }

    // --- Accesseurs ---
    // Permet d'obtenir le pointeur brut.
    const char* get() const {
        return m_ptr;
    }

    // Permet d'utiliser l'objet dans des contextes booléens (ex: if (str)).
    explicit operator bool() const {
        return m_ptr != nullptr;
    }
};


// Nouvelle classe pour la gestion d'un "flag"
class Flag {
private:
    const char* m_ptr = nullptr;

public:
    // Le constructeur acquiert la ressource en appelant la fonction Rust.
    // Le paramètre `salt` est optionnel. S'il est omis, un `nullptr` est passé,
    // ce que notre code Rust interprète comme `None`.
    Flag(const char* step, const char* salt = nullptr)
        : m_ptr(ctflags_format_flag(step, salt)) {}

    // Le destructeur libère la ressource en utilisant la MÊME fonction de libération.
    ~Flag() {
        ctflags_free_string(m_ptr);
    }

    // --- Gestion de la propriété exclusive (identique à RustString) ---
    Flag(const Flag&) = delete;
    Flag& operator=(const Flag&) = delete;

    Flag(Flag&& other) noexcept : m_ptr(other.m_ptr) {
        other.m_ptr = nullptr;
    }
    Flag& operator=(Flag&& other) noexcept {
        if (this != &other) {
            ctflags_free_string(m_ptr);

            m_ptr = other.m_ptr;
            other.m_ptr = nullptr;
        }
        return *this;
    }

    // --- Accesseurs ---
    const char* get() const {
        return m_ptr;
    }
    explicit operator bool() const {
        return m_ptr != nullptr;
    }
};

} // namespace ctflags
#endif  // __CTFLAGS_H_
