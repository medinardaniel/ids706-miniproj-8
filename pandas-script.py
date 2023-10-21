import pandas as pd
import time
from memory_profiler import memory_usage

def calculate() -> pd.DataFrame:
    # Load the CSV data into a pandas DataFrame
    return pd.read_csv("pandas_script/data/data.csv")

def descriptive_statistics(df: pd.DataFrame) -> pd.DataFrame:
    result_data = {}
    
    for col_name in df.columns:
        col = df[col_name]
        
        if col.dtype == 'float64' or col.dtype == 'int64':
            mean_val = col.mean()
            result_data[f'{col_name}_mean'] = [mean_val]
            
            min_val = col.min()
            result_data[f'{col_name}_min'] = [min_val]
            
            max_val = col.max()
            result_data[f'{col_name}_max'] = [max_val]
            
            count_val = len(col)
            result_data[f'{col_name}_count'] = [count_val]
            
    return pd.DataFrame(result_data)

if __name__ == "__main__":
    df = calculate()
    print("Original DataFrame:")
    print(df)
    
    # Calculate runtime
    start_time = time.time()
    try:
        stats = descriptive_statistics(df)
        print("\nDescriptive Statistics:")
        print(stats)
    except Exception as e:
        print(f"Error calculating statistics: {e}")
    end_time = time.time()
    print(f"\nRuntime: {end_time - start_time:.6f} seconds")
    
    # Calculate memory usage
    mem_usage = memory_usage((descriptive_statistics, (df,)))
    print(f"Memory Usage: {max(mem_usage):.6f} MiB")