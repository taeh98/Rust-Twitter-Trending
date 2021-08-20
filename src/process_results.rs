use crate::TweetProcessingResult;

pub fn process_results(algorithm_results: Vec<TweetProcessingResult>) {
    //TODO: write the results to a csv (use polars), make stats measurements (use statrs), and make visualisations of them (use plotters)
    /*
    write results to csv: algorithm name, time taken values (seconds), tweet processing speed values (tweets/second)

    VISUALISATIONS
    bar chart of mean time taken and tweets per second for each algorithm
    box plots of each value of times taken and tweets per second rates for each algorithm
    dot plots of each value of times taken and tweets per second rates for each algorithm
    scatter plot of test number and time taken and test number and tweets/second for each algorithm

    STATS (indented are non-parametric alternatives to above parametric tests)
    mean, median, mode, std dev, variance, IQR of times taken and processing speeds for each algorithm
    independent samples t-tests between times taken and tweets per second rates of all algorithms
        Wilcoxon Rank-Sum tests between times taken and tweets per second rates of all algorithms
    one-way anova tests between times taken and tweets per second rates of all algorithms
        Kruskal Wallis H Tests between times taken and tweets per second rates of all algorithms
    Chi-squared test to see how dependent the categorical variables (Rust or Python and serial or parallel) are
    Phi coefficient to see how associated the categorical variables (Rust or Python and serial or parallel) are
    F-test to compare the variances of the samples
    pearson and spearman correlation coefficients for test number vs time taken for all algorithms

    */
}
