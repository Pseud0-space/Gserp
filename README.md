## Google Search Scraper

This is a Rust project that scrapes search URLs from the first few pages of Google search results for a given query.

## Prerequisites

Before running this project, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager and build tool)

## Getting Started

To get started with this project, follow these steps:

1. Clone the repository: ```git clone https://github.com/Pseud0-space/Gserp.git```
2. Build the project: ```cargo build```
3. Run the project: ```cargo run```

The program will output a vector containing the scraped search URLs.

## Dependencies

This project uses the following dependencies:

- `reqwest`: For making HTTP requests
- `scraper`: For parsing HTML

These dependencies are specified in the `Cargo.toml` file and will be automatically downloaded and installed when you build the project.   
