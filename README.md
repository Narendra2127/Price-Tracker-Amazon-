# Price Scraper for Amazon Products

This project is a web scraper that extracts the product title, price, and timestamp for a specific Amazon product. The data is stored in a CSV file for tracking price changes over time.

## Features

- Scrapes product title and price from Amazon's product page.
- Records the timestamp of when the data was fetched.
- Stores the product data in a CSV file for price tracking.

## Prerequisites

Before running this project, you need to have the following installed:

- [Rust](https://www.rust-lang.org/): The Rust programming language.
- [Reqwest crate](https://crates.io/crates/reqwest): A Rust library for making HTTP requests.
- [Scraper crate](https://crates.io/crates/scraper): A Rust library for parsing HTML.
- [Chrono crate](https://crates.io/crates/chrono): A Rust library for handling date and time.

You can add dependencies in your `Cargo.toml` file like this:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
scraper = "0.12"
chrono = "0.4"
```

## How to Use

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/your-username/price-scraper.git
   ```

2. Navigate to the project folder:

   ```bash
   cd price-scraper
   ```

3. Build and run the project:

   ```bash
   cargo run
   ```

4. The program will scrape the product title and price from the Amazon product URL specified in the code, and append the data to a CSV file.

## Output

The output will look like this:

```
Product Title: Lenovo Gaming Laptop
Price: ₹60,000
Timestamp: 2025-01-01T12:00:00+05:30
Price history updated successfully!
```

A CSV file `price_history_<product_id>.csv` will be created/updated in the project directory containing the following columns:

```
Product Title, Price, Timestamp
```
