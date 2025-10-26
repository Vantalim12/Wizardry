module.exports = {
  apps: [
    {
      name: "collect-fees",
      script: "typescript/dist/src/collect-fees.js",
      cwd: __dirname,
      instances: 1,
      autorestart: true,
      watch: false,
      max_memory_restart: "500M",
      cron_restart: "*/2 * * * *", // Every 2 minutes
      env: {
        NODE_ENV: "production",
      },
      error_file: "./logs/collect-fees-error.log",
      out_file: "./logs/collect-fees-out.log",
      log_date_format: "YYYY-MM-DD HH:mm:ss Z",
      merge_logs: true,
    },
    {
      name: "swap-tokens",
      script: "typescript/dist/src/swap-sol-to-sachi.js",
      cwd: __dirname,
      instances: 1,
      autorestart: true,
      watch: false,
      max_memory_restart: "500M",
      cron_restart: "*/2 * * * *", // Every 2 minutes
      env: {
        NODE_ENV: "production",
      },
      error_file: "./logs/swap-tokens-error.log",
      out_file: "./logs/swap-tokens-out.log",
      log_date_format: "YYYY-MM-DD HH:mm:ss Z",
      merge_logs: true,
    },
    {
      name: "distribute",
      script: "typescript/dist/src/distribute-tokens.js",
      cwd: __dirname,
      instances: 1,
      autorestart: true,
      watch: false,
      max_memory_restart: "1G",
      cron_restart: "*/5 * * * *", // Every 5 minutes
      env: {
        NODE_ENV: "production",
      },
      error_file: "./logs/distribute-error.log",
      out_file: "./logs/distribute-out.log",
      log_date_format: "YYYY-MM-DD HH:mm:ss Z",
      merge_logs: true,
    },
  ],
};
