/*
File: KtorServer.kt
This Kotlin program uses the Ktor framework to start a simple HTTP server.
It defines two endpoints:
1. GET / -> returns "Hello from Ktor!"
2. GET /users -> returns a static JSON list of users

Usage:
    gradle run (if using a Gradle setup)
    or compile and run with a manual classpath that includes Ktor dependencies.

Example Output:
    Server is running on http://localhost:8080
*/

import io.ktor.application.*
import io.ktor.http.*
import io.ktor.response.*
import io.ktor.routing.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.routing.*

fun main() {
    embeddedServer(Netty, port = 8080) {
        routing {
            get("/") {
                call.respondText("Hello from Ktor!", ContentType.Text.Plain)
            }

            get("/users") {
                val users = listOf(
                    mapOf("id" to 1, "name" to "Alice"),
                    mapOf("id" to 2, "name" to "Bob")
                )
                call.respond(users)
            }
        }
    }.start(wait = true)

    println("Server is running on http://localhost:8080")
}
