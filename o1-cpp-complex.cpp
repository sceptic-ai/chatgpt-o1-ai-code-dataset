/*
File: DFS_Graph.cpp
This program implements a graph using adjacency lists and performs a Depth-First Search traversal.
Usage:
    g++ DFS_Graph.cpp -o DFS_Graph
    ./DFS_Graph
Example Output:
    DFS starting from vertex 0:
    0 1 3 2
*/

#include <iostream>
#include <vector>
#include <stack>
using namespace std;

class Graph {
private:
    // Number of vertices
    int V;
    // Adjacency list representation
    vector<vector<int>> adj;

public:
    /**
     * Constructor to initialize the graph with V vertices.
     * 
     * @param vertices The number of vertices in the graph.
     */
    Graph(int vertices) : V(vertices) {
        // Initialize adjacency lists
        adj.resize(vertices);
    }

    /**
     * Adds an edge from src to dest (undirected graph).
     * 
     * @param src The source vertex.
     * @param dest The destination vertex.
     */
    void addEdge(int src, int dest) {
        adj[src].push_back(dest);
        adj[dest].push_back(src); 
    }

    /**
     * Performs a Depth-First Search from a given start vertex using an iterative approach.
     * 
     * @param start The starting vertex for the DFS.
     */
    void DFS(int start) {
        vector<bool> visited(V, false);
        stack<int> stackObj;

        // Push the start vertex
        stackObj.push(start);

        cout << "DFS starting from vertex " << start << ": ";

        while (!stackObj.empty()) {
            int vertex = stackObj.top();
            stackObj.pop();

            // If not visited, mark visited and process
            if (!visited[vertex]) {
                cout << vertex << " ";
                visited[vertex] = true;

                // Push all adjacent vertices that are not visited
                for (auto it = adj[vertex].rbegin(); it != adj[vertex].rend(); ++it) {
                    if (!visited[*it]) {
                        stackObj.push(*it);
                    }
                }
            }
        }
        cout << endl;
    }
};

int main() {
    // Create a graph with 4 vertices (0,1,2,3)
    Graph g(4);
    // Add edges
    g.addEdge(0, 1);
    g.addEdge(0, 2);
    g.addEdge(1, 3);
    g.addEdge(2, 3);

    // Perform DFS starting from vertex 0
    g.DFS(0);

    return 0;
}
