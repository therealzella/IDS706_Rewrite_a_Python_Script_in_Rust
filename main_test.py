import pandas as pd
import pytest
import os

# Test function to check if the dataset loads correctly
def test_load_dataset():
    df = pd.read_csv('cereal.csv')
    # Check if the dataset contains data
    assert not df.empty, "Dataset is empty"

# Test function for summary statistics
def test_summary_statistics():
    df = pd.read_csv('cereal.csv')
    summary_stats = df.describe()

    # Check that the mean of the calories column is calculated
    assert 'calories' in summary_stats.columns, "'calories' column missing in summary stats"
    assert summary_stats['calories']['mean'] > 0, "Calories mean should be greater than 0"

def test_histogram_creation():
    # Get the absolute path of the histogram file
    current_dir = os.path.dirname(os.path.abspath(__file__))
    histogram_path = os.path.join(current_dir, 'calories_histogram.png')
    
    # Ensure that the histogram image file is created
    assert os.path.exists(histogram_path), "Histogram image not created at: " + histogram_path
