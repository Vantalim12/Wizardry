import { execSync } from "child_process";
import * as dotenv from "dotenv";
import * as path from "path";

// Load environment variables
dotenv.config();

async function main() {
  console.log("Starting fee collection process...");

  const rustBinaryPath = path.join(
    __dirname,
    "../../rust/target/release/collect_fees"
  );

  try {
    // Run the Rust binary
    execSync(rustBinaryPath, {
      stdio: "inherit",
      env: process.env,
    });

    console.log("Fee collection completed successfully");
  } catch (error) {
    console.error("Fee collection failed:", error);
    process.exit(1);
  }
}

main();
