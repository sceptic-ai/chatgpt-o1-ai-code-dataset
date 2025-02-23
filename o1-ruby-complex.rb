# File: ruby_scraper.rb
# This script uses Nokogiri and open-uri to scrape all links (<a> tags) from a webpage.
# Usage:
#   gem install nokogiri
#   ruby ruby_scraper.rb
# Example Output:
#   Found 5 links on the page.
#   Link #1 -> https://example.com
#   ...

require 'nokogiri'
require 'open-uri'

def scrape_links(url)
  # Open the URL and parse with Nokogiri
  doc = Nokogiri::HTML(URI.open(url))
  
  # Find all <a> tags and map their href attributes
  links = doc.css('a').map { |link| link['href'] }.compact
  links
end

# Example usage
if __FILE__ == $0
  begin
    example_url = "https://example.com"
    links_found = scrape_links(example_url)
    puts "Found #{links_found.size} links on the page."
    links_found.each_with_index do |lnk, idx|
      puts "Link ##{idx + 1} -> #{lnk}"
    end
  rescue StandardError => e
    puts "An error occurred: #{e.message}"
  end
end
