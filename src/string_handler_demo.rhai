// string_handler_demo.rhai

// Create a new StringHandler instance with initial content
let s = new_string("World");

// Append a string to the end
s.append("! Welcome to the Rhai-Rust integration project.");

// Prepend a string to the beginning
s.prepend("Hello, ");

// Return the final string content
return s.content;

