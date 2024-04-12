// Computes the n-th Fibonacci number using recursive method
fn fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

// Main entry for the performance test with a high number for intensive computation
// fibonacci(35)
fibonacci(3)
