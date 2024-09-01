# Install dependencies from requirements.txt
install:
	pip install -r requirements.txt

# Format Python files using black
format: 
	black *.py

# Check if Python files are formatted correctly (without modifying them)
check-format: 
	black --check *.py

# Lint Python files using pylint
lint: 
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py

# Run tests using pytest and measure code coverage
test: 
	python -m pytest --cov=main test_main.py

# Generate and display a code coverage report in HTML format
coverage: 
	python -m pytest --cov=main --cov-report=html
	@echo "Open htmlcov/index.html to view the coverage report."

# Clean up temporary files and directories
clean: 
	@# Find and delete all Python bytecode files and __pycache__ directories
	find . -type f -name '*.py[co]' -delete
	find . -type d -name '__pycache__' -exec rm -rf {} +
	@# Remove pytest cache directories
	rm -rf .pytest_cache

# Create a virtual environment
venv: 
	python3 -m venv venv
	@echo "Virtual environment created. Activate it using 'source venv/bin/activate' (Linux/Mac) or 'venv\Scripts\activate' (Windows)."

# Activate the virtual environment
activate: 
	@source venv/bin/activate

# Update the requirements.txt file with the current environment's packages
requirements: 
	pip freeze > requirements.txt

# Build a distribution package
dist: 
	python setup.py sdist bdist_wheel

# Run all major targets: install, format, lint, test
all: install format lint test

# The following targets are designed to work on Unix-based systems.
# If you're on Windows, you may need to adjust the commands, especially for the 'clean' target.
# To run a specific target, use 'make <target_name>'. For example, 'make install' to install dependencies.


