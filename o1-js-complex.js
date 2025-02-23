/*
File: server.js
This code sets up a simple REST API using Express.js. It defines two routes:
1. GET / - returns a welcome message
2. GET /users - returns a static list of users
Usage:
    npm install express
    node server.js
Example Output:
    Server listening on port 3000
    (Visiting http://localhost:3000/ in a browser displays "Welcome to our API!")
*/

const express = require('express');
const app = express();
const PORT = 3000;

/**
 * GET / - Root endpoint.
 * Sends a simple welcome message.
 */
app.get('/', (req, res) => {
  res.send('Welcome to our API!');
});

/**
 * GET /users - Returns a static JSON list of users.
 */
app.get('/users', (req, res) => {
  const users = [
    { id: 1, name: 'Alice', age: 29 },
    { id: 2, name: 'Bob', age: 34 },
    { id: 3, name: 'Charlie', age: 25 },
  ];
  res.json(users);
});

// Start the server
app.listen(PORT, () => {
  console.log(`Server listening on port ${PORT}`);
});
