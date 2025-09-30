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
      text: 'Markdown Tables',
      link: '/markdown-tables',
    },
    {
      text: 'Benchmark Results',
      link: '/benchmark-results',
    },
    {
      text: 'Example',
      link: '/example',
    },
  ],
})
