# Statistical Analysis Reports

This directory contains statistical analysis reports that provide detailed insights into profiling results through mathematical analysis and data visualization.

## Available Statistical Reports

*No statistical reports available yet. Generate statistical reports using the analysis scripts.*

## Types of Statistical Analysis

### Descriptive Statistics
- **Mean, Median, Mode**: Central tendency measures
- **Standard Deviation**: Variability measures
- **Min/Max Values**: Range analysis
- **Percentiles**: Distribution analysis

### Performance Analysis
- **Resource Usage Patterns**: How resources scale with gas categories
- **Efficiency Metrics**: Resource usage per unit of work
- **Bottleneck Identification**: Areas of high resource consumption
- **Optimization Opportunities**: Areas for improvement

### Comparative Analysis
- **Statistical Significance**: Whether differences between implementations are meaningful
- **Correlation Analysis**: Relationships between different metrics
- **Trend Analysis**: Performance changes over time
- **Outlier Detection**: Unusual performance characteristics

## Generating Statistical Reports

### Basic Statistical Analysis
```bash
python3 scripts/generate_markdown_tables.py --statistics \
  zkevm-metrics-risc0-10M \
  --output benchmark-results/markdown-reports/statistics/risc0-10M-statistics.md
```

### Comprehensive Statistical Analysis
```bash
./scripts/generate_results.sh --compare --statistics \
  --output benchmark-results/markdown-reports/statistics/comprehensive-statistics.md
```

### Custom Statistical Analysis
```bash
# Generate statistics for specific gas categories
python3 scripts/generate_markdown_tables.py --statistics --gas-categories \
  zkevm-metrics-risc0-1M zkevm-metrics-risc0-10M zkevm-metrics-risc0-100M \
  --output benchmark-results/markdown-reports/statistics/gas-category-statistics.md
```

## Statistical Metrics Explained

### Execution Metrics
- **Total Cycles**: Computational complexity measure
- **Duration**: Time efficiency measure
- **Region Cycles**: Breakdown of computational phases

### Proving Metrics
- **Proof Size**: Proof generation efficiency
- **Proving Time**: Proof generation speed
- **Memory Usage**: Resource consumption patterns

### Comparative Metrics
- **Speedup Ratios**: Performance improvement measures
- **Efficiency Gains**: Resource optimization measures
- **Scalability Factors**: Performance scaling characteristics

## File Naming Convention

- **Single Category**: `{ZKVM}-{GAS_CATEGORY}-statistics.md`
- **Multiple Categories**: `{DESCRIPTION}-statistics.md`
- **Comprehensive**: `comprehensive-statistics.md`
- **Custom Analysis**: `{ANALYSIS_TYPE}-statistics.md`

## Analysis Workflow

1. **Data Collection**: Run profiling to collect raw metrics
2. **Statistical Processing**: Generate statistical summaries
3. **Pattern Recognition**: Identify performance patterns
4. **Insight Generation**: Extract actionable insights
5. **Report Creation**: Document findings and recommendations

## Key Insights to Look For

### Performance Patterns
- **Linear Scaling**: How performance scales with gas categories
- **Non-linear Behavior**: Areas where performance doesn't scale linearly
- **Efficiency Plateaus**: Points where efficiency stabilizes

### Resource Utilization
- **Memory Efficiency**: How well memory is utilized
- **CPU Efficiency**: How well computational resources are used
- **Storage Efficiency**: How well storage resources are managed

### Optimization Opportunities
- **High Resource Usage**: Areas consuming excessive resources
- **Inefficient Patterns**: Suboptimal resource utilization
- **Bottleneck Identification**: Performance-limiting factors

## Integration with Other Reports

Statistical reports work together with:
- **Main Profiling Results**: Provide context for raw data
- **Comparison Reports**: Offer statistical validation of differences
- **Latest Results**: Track performance trends over time

## Best Practices

### Statistical Analysis
1. **Use Appropriate Metrics**: Choose metrics relevant to your analysis
2. **Consider Sample Size**: Ensure sufficient data for reliable statistics
3. **Account for Variability**: Consider standard deviation and confidence intervals
4. **Validate Assumptions**: Check that statistical assumptions are met

### Report Generation
1. **Include Context**: Provide background for statistical findings
2. **Explain Methodology**: Document how statistics were calculated
3. **Highlight Key Findings**: Emphasize the most important insights
4. **Provide Recommendations**: Suggest actions based on findings

### Data Quality
1. **Verify Data Integrity**: Ensure profiling results are accurate
2. **Handle Outliers**: Identify and explain unusual data points
3. **Check Consistency**: Verify that results are internally consistent
4. **Document Limitations**: Note any limitations in the analysis
