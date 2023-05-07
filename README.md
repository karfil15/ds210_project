# ds210_project
A graph-based model which analyzes the relationship between the interest rates and inflation
Project Summary

Overview

This project explores the relationship between inflation rates and interest rates using historical financial data. 
The main objective is to analyze whether there is a proportional relationship between the two variables. 
The data is represented in the form of a graph, where each node represents a specific time period with its associated inflation rate and interest rate. 
Edges between nodes represent the similarity in inflation and interest rates between adjacent time periods.

Data

The dataset consists of two CSV files containing monthly inflation rates and interest rates, respectively, from 1969 to 2021. 
The data is obtained from the OECD database.

Methodology

1.	Data from the two CSV files is merged to create a combined dataset of data points with the corresponding date, inflation rate, and interest rate.

2.	A directed graph is constructed, where each node (vertex) represents a data point with its associated date, inflation rate, and interest rate. 
Edges are created between nodes if the difference between their interest rates and inflation rates is within a predefined threshold 
(the threshold was added for testing purposes but it was decided it should remain in the final code).

3.	The graph is analyzed to calculate the average edge distance and the degree distribution of nodes.

Results

The analysis of the constructed graph yields the following results:

•	Average edge distance: 0.4313
•	Degree distribution: {1: 623, 0: 1}

These results provide insights into the relationship between inflation rates and interest rates over time. 
The average edge distance indicates the average similarity between adjacent time periods in terms of their inflation and interest rates. 
In this case, the average edge distance is 0.4313, which indicates that on average, there is a 0.4313 unit change in the combination 
of inflation and interest rates from month to month. This value helps to understand the overall volatility in the relationship between inflation and interest rates.

The degree distribution shows the number of nodes with a specific number of edges, revealing the overall connectivity of the graph. 
In this case, the degree distribution shows that 623 nodes (months) have one edge connecting them to another node 
(i.e., they have one consecutive month with a change in inflation and interest rates less than or equal to the threshold), while one node has no edges. 
This result indicates that, in most cases, there is a connection between consecutive months, and the changes in inflation and interest rates are below the threshold.
