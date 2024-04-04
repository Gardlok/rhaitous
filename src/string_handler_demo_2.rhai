// advanced_string_handler_demo.rhai

// Create a new StringHandler instance with initial content
let handler = new_string("Rhai and Rust Integration: ");

// Use a loop to append numbers with conditions
for i in range(0, 10) {
    // Check if the number is even
    if i % 2 == 0 {
        handler.append("Even ");
    } else {
        // For odd numbers, prepend to demonstrate both operations
        handler.prepend("Odd ");
    }
}

// Append a concluding statement
handler.append(" - Demonstrating advanced string manipulation!");

// Return the final string content
return handler.content;

