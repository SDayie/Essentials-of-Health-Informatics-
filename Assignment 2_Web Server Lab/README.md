
# Programming Assignment 2: Web Server Lab

## How the Web Server Works

1. **Browser asks for a webpage:** The browser requests a file called `HelloWorld.html`.

2. **Python server receives the request:** The server listens for the browser and reads its request.

3. **Server searches for the file:** It looks for `HelloWorld.html` in its folder.

4. **If the file exists:** The server sends the webpage to the browser, and the browser displays it.

5. **If the file does not exist:** The server sends a `404 Not Found` error message to the browser.

![alt text](<assoignment 2.png>)

## Running the Web Server

### Step 1: Start the server

Open Terminal in the project folder and run:

```bash
python3 server.py
```

Keep Terminal open while testing.

### Step 2: Open the webpage

Open Safari or Chrome and visit:

http://127.0.0.1:6789/HelloWorld.html

![alt text](<Hello World.png>)

The browser should display the Hello World webpage.

### Step 3: Test the 404 response

Visit:

http://127.0.0.1:6789/missing.html

![alt text](<404 Not Found.png>)

The server should return `404 Not Found`.

### Step 4: Stop the server

Return to Terminal and press **Control + C**.


