"""
File: web_scraper.py
This script fetches the HTML content of a given URL and scrapes specific information (example: all <h2> headings).
Usage:
    pip install requests beautifulsoup4
    python web_scraper.py
Example Output:
    Found 3 <h2> headings on the page.
"""

import requests
from bs4 import BeautifulSoup

def scrape_headings(url):
    """
    Fetch the HTML content from the given URL and parse <h2> elements.
    
    Args:
        url (str): A valid URL to scrape (must include http or https).
    Returns:
        list: A list of strings representing the text of each <h2> heading found.
    """
    # Send a GET request to the specified URL
    response = requests.get(url)
    
    # Raise HTTPError if the request was unsuccessful
    response.raise_for_status()
    
    # Parse the HTML content using BeautifulSoup
    soup = BeautifulSoup(response.text, "html.parser")
    
    # Find all <h2> tags
    h2_tags = soup.find_all("h2")
    
    # Extract text from each <h2> element
    headings = [tag.get_text(strip=True) for tag in h2_tags]
    
    return headings

def main():
    # Example URL for demonstration purposes
    example_url = "https://example.com"
    
    # Scrape headings
    headings_list = scrape_headings(example_url)
    
    # Print the results
    print(f"Found {len(headings_list)} <h2> headings on the page.")
    for i, heading in enumerate(headings_list, 1):
        print(f"{i}. {heading}")

if __name__ == "__main__":
    main()
