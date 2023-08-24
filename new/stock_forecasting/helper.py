import torch
from torch.utils.data import Dataset
import numpy as np

# Max-Min Normalization
def normalize(data: np.array) -> (np.array, float, float):
    max_val = np.max(data)
    min_val = np.min(data)
    return (data - min_val) / (max_val - min_val), max_val, min_val

def sliding_window(data: np.array, window_size: int, horizon: int):
    x = []
    y = []
    for i in range(len(data) - window_size - horizon + 1):
        x.append(data[i:i+window_size])
        y.append(data[i+window_size:i+window_size+horizon])
    return np.array(x), np.array(y)

def train_test_split(x: np.array, y: np.array, train_ratio: float):
    train_size = int(len(x) * train_ratio)
    return x[:train_size], x[train_size:], y[:train_size], y[train_size:]

def recover_normalize(x, max_, min_):
    return x * (max_ - min_) + min_

class StockDataset(Dataset):
    def __init__(self, X, y):
        self.X = torch.tensor(X, dtype=torch.float32)
        self.y = torch.tensor(y, dtype=torch.float32)
    
    def __getitem__(self, index):
        return self.X[index], self.y[index]
    
    def __len__(self):
        return len(self.X)

