import ctflags

def main():
    # Test the seed
    seed = ctflags.get_seed_or_null()
    if seed:
        print(f"Seed is : {seed}")

    # Test the flag
    flag = ctflags.format_flag("example")
    print(f"Example flag: {flag}")

    # Salted flag
    flag_salt = ctflags.format_flag("example", "mon_sel")
    print(f"Flag with salt: {flag_salt}")

    # Test with a string context
    with_string_context = ctflags.format_flag_from_context("segg1545", "example")
    print(f"Flag with string context is {with_string_context}")
    assert with_string_context == "flag(example).5f1b958992ca66c09c0ac9170fce85de"

if __name__ == "__main__":
    main()
