import json
import os
from sys import platform


def main():
    discord_token = input("Enter your discord bot token: ")
    openai_key = input("Enter your OpenAI API key: ")

    print("\nWriting config.json...")

    with open("config.json", "w") as f:
        json.dump({
            "discord_token": discord_token,
            "openai_key": openai_key
        }, f)

    print("Creating directories...")

    os.mkdir("tmp")
    os.mkdir("guilds")

    print("Downloading Rust if not already downloaded...")

    if os.system("cargo --version") != 0:
        if platform == "linux":
            os.system(
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            print("Rust installed!")

        elif platform == "win32":
            print("Looks like you are running Windows. Please go to https://rustup.rs/# and install Rust manually.")

    else:
        print("Rust already installed!")

    print("Setup complete!")


if __name__ == "__main__":
    main()
