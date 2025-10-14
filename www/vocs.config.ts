import { defineConfig } from 'vocs'

export default defineConfig({
  title: 'zkGas profiling',
  description: 'Comprehensive profiling framework for measuring and comparing the resources needed for proving different OPCODEs in zk environments across various gas categories.',
  sidebar: [
    {
      text: 'Getting Started',
      link: '/getting-started',
    },
    {
      text: 'Download Fixtures',
      link: '/download-fixtures',
    },
    {
      text: 'Gas Categorized Fixtures',
      link: '/gas-categorized-fixtures',
    },
    {
      text: 'Gas Categorized Benchmarks',
      link: '/gas-categorized-benchmarks',
    },
    {
      text: 'Single File Benchmark',
      link: '/single-file-benchmark',
    },
    {
      text: 'Markdown Tables',
      link: '/markdown-tables',
    },
    {
      text: 'Simplified Naming',
      link: '/simplified-naming',
    },
    {
      text: 'Benchmark Results',
      items: [
        {
          text: '1M, sp1',
          link: '/benchmark-results/gas-categorized/1m-sp1',
        },
        {
          text: '1M, risc0',
          link: '/benchmark-results/gas-categorized/1m-risc0',
        },
      ]
    },
  ],
})
