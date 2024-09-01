def factorial(n):
    """
    Calculate the factorial of a non-negative integer n.
    Raise a ValueError if n is negative.
    """
    if n < 0:
        raise ValueError("Factorial is not defined for negative numbers.")
    if n == 0 or n == 1:
        return 1
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


if __name__ == "__main__":
    # Example usage
    try:
        number = 5
        result = factorial(number)
        print(f"The factorial of {number} is {result}")
    except ValueError as e:
        print(e)
