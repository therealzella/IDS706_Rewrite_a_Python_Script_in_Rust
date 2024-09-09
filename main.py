import pandas as pd
import matplotlib.pyplot as plt

# Load the 'cereal.csv' dataset
df = pd.read_csv('cereal.csv')

# Generate summary statistics
summary_stats = df.describe()

# Print summary statistics to the console
print(summary_stats)

# Save the summary statistics to a CSV file
summary_stats.to_csv('summary_statistics.csv')

# Create a histogram of one of the columns (e.g., 'calories')
df['calories'].hist(bins=10, figsize=(10, 8))
plt.title('Distribution of Calories')
plt.xlabel('Calories')
plt.ylabel('Frequency')
plt.savefig('calories_histogram.png')

print("Summary statistics and calories histogram have been generated.")


