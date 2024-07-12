import pandas as pd

# Load the existing Excel file
file_path = './'
xls = pd.ExcelFile(file_path)


sheet_names = xls.sheet_names
sheet_names

benchmark_data = {
    'Server': [
        'NodeJS single thread',
        'NodeJS Cluster Max conn -c1800',
        'Go Net-Http max conn -c1900',
        'Java Single Thread max conn -c1500',
        'Java multithreads max conn -c3100',
        'Rust axtic-web max conn -c2800'
    ],
    'Avg Latency (ms)': [
        11.73, 21.31, 12.61, 25.87, 4.19, 15.32
    ],
    'Stdev Latency (ms)': [
        3.40, 30.51, 16.48, 66.64, 15.26, 12.50
    ],
    'Max Latency (ms)': [
        188.22, 312.04, 216.90, 419.23, 370.46, 179.72
    ],
    '+/- Stdev (%)': [
        98.39, 87.16, 86.15, 92.71, 98.85, 87.34
    ],
    'Req/Sec Avg': [
        2670, 2800, 4280, 1600, 4000, 2300
    ],
    'Req/Sec Stdev': [
        163.34, 667.43, 705.95, 383.54, 1550, 183.18
    ],
    'Req/Sec Max': [
        4000, 5480, 8610, 3730, 16230, 5470
    ],
    '+/- Stdev Req/Sec (%)': [
        91.02, 68.67, 72.27, 69.69, 71.02, 82.21
    ],
    'Total Requests': [
        1275960, 1340177, 2047231, 765378, 1893371, 1099075
    ],
    'Data Read (MB)': [
        771.48, 810.31, 1080, 414.60, 1024, 776.75
    ],
    'Requests/sec': [
        42484.87, 44604.86, 68129.48, 25482.41, 62900.53, 36508.35
    ],
    'Transfer/sec (MB)': [
        25.69, 26.97, 36.90, 13.80, 34.07, 25.80
    ],
    'Socket Errors: Connect': [
        0, 0, 0, 0, 0, 0
    ],
    'Socket Errors: Read': [
        1601, 356, 689, 8559, 23281, 667
    ],
    'Socket Errors: Write': [
        1, 0, 0, 288, 178, 0
    ],
    'Socket Errors: Timeout': [
        0, 0, 0, 0, 0, 0
    ]
}

# Convert the dictionary to a DataFrame
benchmark_df = pd.DataFrame(benchmark_data)

# Load the existing Excel file
with pd.ExcelWriter(file_path, engine='openpyxl', mode='a') as writer:
    # Write the new data to a new sheet named 'Benchmarks'
    benchmark_df.to_excel(writer, index=False, sheet_name='Benchmarks')

# Load the updated Excel file to verify the new sheet
updated_xls = pd.ExcelFile(file_path)
updated_xls.sheet_names
