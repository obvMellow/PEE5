import json, os

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
        os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")

        print("Rust installed!")
    else:
        print("Rust already installed!")

    print("Setup complete!")