# import necessary modules
import requests, bs4, os

# request
res = requests.get('https://math.tools/numbers/pi/10')

# gain all html-code
soup = bs4.BeautifulSoup(res.text, 'html.parser')

# CSS-selector
Elems = soup.find('div', class_='pidata')

for elem in Elems.contents:
    print(elem[1:-1])