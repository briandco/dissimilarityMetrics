**Dissimilarity Metrics Calculator**

Overview

The Dissimilarity Metrics Calculator is a desktop application built using Rust and egui (a simple, fast, and flexible GUI library). It provides a convenient way to compare two files and calculate various dissimilarity metrics such as Levenshtein Distance, Cosine Similarity, and Percentage Change.

This tool is particularly useful for comparing code files, documents, or any textual content to analyze how similar or different they are. It offers a clean graphical interface, allowing users to easily select files and view results in a clear, structured manner.
Features

    File Selection: Allows users to select two files for comparison using a graphical file picker.
    Levenshtein Distance Calculation: Calculates the Levenshtein distance between corresponding segments in the two selected files. This metric shows how many changes (inserts, deletes, or substitutions) are needed to transform one file into the other.
    Cosine Similarity: Measures the cosine similarity between the two files, providing insight into how similar the content is based on vector space modeling.
    Change Percentage: Shows the percentage difference between the files, indicating the extent of changes.
    Visual Representation: Displays the comparison results in a table format with key metrics for easy interpretation.
    Reset Functionality: Users can reset the application to clear file selections and results.

How It Works

    Select two files using the "Select File 1" and "Select File 2" buttons.
    Once both files are selected, click on the "Calculate Levenshtein Distance" button to compute the dissimilarity metrics.
    The application will then:
        Extract textual content from the selected files.
        Compute Levenshtein Distance, Cosine Similarity, and Change Percentage for each corresponding section.
        Display these results in a table format.
    View detailed results in the main table, which includes key metrics for every section of the files.
