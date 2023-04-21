import json
import os
from sys import platform
from getpass import getpass


def main():
    print("Welcome to the PEE5 setup script!\nAll necessary files will be downloaded and installed automatically.\nRun this script inside the directory you want to install PEE5 in.\n")

    discord_token = getpass("Enter your discord bot token: ")
    openai_key = getpass("Enter your OpenAI API key: ")

    print("\nDownloading Rust...\n")

    if os.system("cargo --version") != 0:
        if platform == "linux":
            os.system(
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            print("Rust installed!")

        elif platform == "win32":
            print("Looks like you are running Windows. Please go to https://rustup.rs/# and install Rust manually.")
        
        else:
            print("Your operating system is not supported. Please install Rust manually.")
            input("Press enter after you installed Rust...")

    else:
        print("Rust already installed!")

    source = input("Do you want to compile from source? (y/N): ")

    if source == "y":
        print("\nCompiling from source...\n")

        print("Cloning repository...")
        if os.system("git clone https://github.com/obvMellow/PEE5.git") != 0:
            print("Failed to clone repository. Make sure \"git\" is installed.")
            return
        os.chdir("PEE5")

        print("Building...")

        if os.system("cargo build --release") != 0:
            print("Failed to compile from source. Please try again.")
            return

        os.system("mv target/release/pee5 ..")
        os.chdir("..")

        print("Cleaning up...")
        os.system("rm -rf PEE5")

        print("\nCompiled successfully!")

    else:
        print("\nDownloading precompiled binary...")

        if platform == "linux":
            os.system("wget https://github.com/obvMellow/PEE5/releases/download/v1.0.0/pee5-linux-1_0_0")
            os.rename("pee5-linux-1_0_0", "pee5")
            os.system("chmod +x pee5")

            print("Downloaded successfully!")

        else:
            print("Your operating system is not supported. Please compile from source.")
            return

    print("\nWriting config.json...")

    with open("config.json", "w") as f:
        json.dump({
            "discord_token": discord_token,
            "openai_key": openai_key
        }, f)

    print("Creating directories...")

    os.mkdir("tmp")
    os.mkdir("guilds")
    os.mkdir("saved_imagines")

    print("\nSetup complete!")


if __name__ == "__main__":
    main()
