import FinanceDataReader as fdr
import numpy as np
import pandas as pd

df = fdr.DataReader('005930', '2000-01-01', pd.Timestamp.today().strftime('%Y-%m-%d'))

# Extraact date & Close
dg = pd.DataFrame({
    'date': df.index.strftime('%Y-%m-%d').to_list(),
    'close': df['Close'].to_list(),
    'high': df['High'].to_list(),
    'low': df['Low'].to_list(),
})

# Save data to "data/{code}/close.parquet"
# if folder "data/{code}" does not exist, create it
dg.to_parquet("data/close.parquet")
