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

if __name__ == "__main__":
    main()
