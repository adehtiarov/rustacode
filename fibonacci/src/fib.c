#include <stdio.h>

unsigned long long fib(int n) {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
int main() {
    int n;
    printf("Enter a number: ");
    scanf("%d", &n);
    printf("Fibonacci of %d is %llu\n", n, fib(n));
    return 0;
}