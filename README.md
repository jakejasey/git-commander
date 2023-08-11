# Git Commander 

Overview
Git-Commander is a powerful and intuitive command-line tool designed to provide seamless remote management and control of Git repositories. This open-source project aims to enhance developers' productivity by offering a streamlined interface that empowers them to interact with Git repositories from anywhere in the world, all without leaving their beloved terminal environment.

Features
1. Remote Repository Management
With Git-Commander, developers can effortlessly create, clone, fork, and delete remote Git repositories. The tool abstracts away the complexities of working with different repository hosting platforms, supporting popular services like GitHub, GitLab, and Bitbucket. It offers a unified experience, regardless of the hosting platform.

2. Branch Operations
Git-Commander simplifies branching operations, allowing users to easily create, switch, merge, and delete branches remotely. Effortlessly manage your codebase's branches directly from the command line, reducing the need for manual intervention and providing a more efficient workflow.

3. Collaborative Workflows
Collaboration is at the heart of Git-Commander. Developers can invite contributors, review pull requests, and manage access permissions to ensure a secure and efficient development process. Say goodbye to switching between the browser and the terminal—Git-Commander puts all collaborative tasks within easy reach.

4. Commit and Push
Stay in the zone and never lose momentum. Git-Commander allows you to stage, commit, and push changes to your remote repository with a few simple commands. It keeps the version control process swift and straightforward, enabling you to focus on coding.

5. Code Review
Efficiently review code changes and provide feedback from the command line. Git-Commander streamlines code review tasks, making it easy to accept or reject pull requests and collaborate with your team without disrupting your workflow.

6. Seamless Integration
Integrate Git-Commander seamlessly into your existing development toolkit. The tool supports all major platforms and can be easily integrated with your preferred IDEs and text editors. Git-Commander also plays well with popular CI/CD tools, ensuring a smooth and efficient development-to-deployment pipeline.

Installation
To get started with Git-Commander, simply follow our easy installation guide in the Wiki. We provide step-by-step instructions for different operating systems to ensure a hassle-free setup.

Getting Started
Visit our Documentation to learn how to use Git-Commander effectively. It includes comprehensive guides, usage examples, and troubleshooting tips to help you master the tool quickly.


Because I am creating this project as a learning experience to better understand the Rust ecosystem, I will provide a detailed discriiption of how this code initialy works. 

Imports and dependencies:

The program uses the reqwest library for making HTTP requests.
It uses serde for serializing and deserializing JSON data.
std::env is for environment variable access.
std::error::Error provides a general-purpose error type.
Data Structures:

NewRepo: A struct representing the data needed to create a new GitHub repository.
It has a lifetime parameter 'a which denotes the lifespan of references within the struct.
It contains three fields: name, description, and private.
RepoResponse: A struct representing the response from GitHub after trying to create a repo. This is used to deserialize the JSON response.
The create_github_repo function:

This asynchronous function takes in a GitHub token (as a string slice) and a NewRepo struct.
A new reqwest client is created.
Headers are set up for the HTTP request. Specifically:
The User-Agent is set to "reqwest".
The Authorization header is set using the provided GitHub token.
The function sends a POST request to GitHub's repo creation API endpoint.
If the request is successful, it prints the URL of the newly created repository.
If the request fails, it prints the failed status and the error message.
The get_user_input function:

This function displays a given prompt to the user and reads their input from the standard input.
It returns the trimmed user input as a String.
The main function:

Annotated with #[tokio::main], it signifies this function as the asynchronous entry point for the program.
The program first checks for the existence of the GITHUB_TOKEN environment variable.
It then uses get_user_input to get the name and description for the new repo from the user.
A NewRepo struct is created with the provided name and description, and a fixed private value of false (indicating the repo should be public).
The create_github_repo function is called with the GitHub token and the NewRepo struct to try and create the repo.
To summarize, this program is a simple command-line utility to create a new GitHub repository. When run, it prompts the user for the name and description of a new public repository and then attempts to create it using the GitHub API and a token provided as an environment variable. If successful, it will print the URL of the new repository. If unsuccessful, it will print an error message.



