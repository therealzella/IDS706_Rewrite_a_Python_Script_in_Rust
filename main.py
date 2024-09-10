import pandas as pd
import matplotlib.pyplot as plt
import os

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

# Save the histogram in the same directory as the script
current_dir = os.path.dirname(os.path.abspath(__file__))
histogram_path = os.path.join(current_dir, 'calories_histogram.png')
plt.savefig(histogram_path)

print("Summary statistics and calories histogram have been generated at:", histogram_path)



