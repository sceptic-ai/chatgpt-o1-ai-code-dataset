//
// File: UserModelView.swift
// This demonstrates a simple MV pattern in Swift, using a command-line approach
// instead of an actual iOS UI. A User model and a View simulate the data flow.
// Usage:
//   swift UserModelView.swift
// Example Output:
//   John Doe is 30 years old.
//   Changing age to 31...
//   John Doe is 31 years old.
//

import Foundation

// Model: Holds user data
struct User {
    var name: String
    var age: Int
}

// View: Displays user data (simulated in console)
struct UserView {
    func displayUserInfo(user: User) {
        print("\(user.name) is \(user.age) years old.")
    }
}

// Controller (or ViewModel in a more MVVM approach): 
// Binds model and view together
struct UserController {
    private var user: User
    private var view: UserView
    
    init(user: User, view: UserView) {
        self.user = user
        self.view = view
    }
    
    mutating func updateUserAge(to newAge: Int) {
        user.age = newAge
    }
    
    func display() {
        view.displayUserInfo(user: user)
    }
}

var myUser = User(name: "John Doe", age: 30)
let myView = UserView()
var myController = UserController(user: myUser, view: myView)

// Display initial user info
myController.display()

// Update user's age
print("Changing age to 31...")
myController.updateUserAge(to: 31)

// Display updated info
myController.display()
