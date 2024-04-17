# Arkham_API_test 🦇

This Rust project demonstrates how to securely configure and run an application using environment variables for API keys. It's designed to interact with external APIs securely without hardcoding sensitive information.

## Installation 🔧

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/foxxcn/Arkham_API_test.git
cd Arkham_API_test
```

## Configuration 🔑

Create a `.env` file in the root directory and add your API key:

```plaintext
API_KEY=your_actual_api_key_here
```

Ensure that the `.env` file is listed in your `.gitignore` to avoid pushing it to your repository.

## Usage 🚀

Run the project using Cargo, which automatically loads the environment variables from the `.env` file:

```bash
cargo run
```

This command will start the application, which will use the API key stored in the `.env` file to make requests to the configured API.

## License 📄

Distributed under the MIT License. See `LICENSE` for more information.
