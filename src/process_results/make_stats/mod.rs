/*
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

use crate::TweetProcessingResult;

//TODO: implement this with statrs (?)
pub fn make_stats(algorithm_results: &Vec<TweetProcessingResult>) {
    println!("make_stats()");
}
